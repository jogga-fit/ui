use base64::{Engine as _, engine::general_purpose::STANDARD};
use dioxus::prelude::*;

#[css_module("/src/pages/profile/style.css")]
struct Styles;

use crate::{
    CheckFollowingFn, DeleteObjectFn, FollowArgs, FollowPersonFn, GetActorConnectionsArgs,
    GetActorConnectionsFn, GetActorInfoFn, GetActorPostsArgs, GetActorPostsFn, LikeFn, RouteFn,
    TokenApIdArgs, UnfollowActorFn, UpdatePostFn, UpdateProfileArgs, UpdateProfileFn,
    UploadAvatarArgs, UploadAvatarFn,
    browser::copy_to_clipboard,
    components::{
        avatar::{Avatar, AvatarSize},
        crop_modal::{CropModal, CropModalState},
        error_banner::ErrorBanner,
        post::FeedCard,
    },
    image::{
        CropSelection, clear_file_input, compress_avatar_from_input,
        prepare_selected_image_from_input, revoke_object_url,
    },
    sleep_ms,
    types::{ActorInfo, AuthSignal, ConnectionItem, ConnectionsResult},
};

#[component]
pub fn ProfilePageView(
    username: String,
    get_actor_info_fn: GetActorInfoFn,
    get_actor_posts_fn: GetActorPostsFn,
    get_actor_connections_fn: GetActorConnectionsFn,
    check_following_fn: CheckFollowingFn,
    update_profile_fn: UpdateProfileFn,
    upload_avatar_fn: UploadAvatarFn,
    follow_person_fn: FollowPersonFn,
    unfollow_actor_fn: UnfollowActorFn,
    delete_fn: DeleteObjectFn,
    like_fn: LikeFn,
    unlike_fn: LikeFn,
    route_fn: RouteFn,
    update_fn: UpdatePostFn,
) -> Element {
    let auth = use_context::<AuthSignal>();

    // Peek-compare: sync prop into signal so use_resource re-runs on navigation.
    // (use_effect only fires on signal changes, not prop-change re-renders.)
    let plain = username.trim_start_matches('@').to_string();
    let mut plain_sig = use_signal(|| plain.clone());
    if *plain_sig.peek() != plain {
        plain_sig.set(plain.clone());
    }

    let info = use_resource(move || {
        let p = plain_sig();
        async move { get_actor_info_fn.call(p).await }
    });

    // Gate on `ready` so SSR and initial client render produce identical output.
    // After hydration the effect fires and re-renders with correct state.
    let mut ready = use_signal(|| false);
    use_effect(move || {
        ready.set(true);
    });
    let is_own = *ready.read()
        && auth
            .read()
            .as_ref()
            .map(|u| {
                let p = plain_sig.read();
                let local = p.split_once('@').map(|(u, _)| u).unwrap_or(&p);
                u.username == local
            })
            .unwrap_or(false);
    let is_logged_in = *ready.read()
        && auth
            .read()
            .as_ref()
            .map(|u| !u.token.is_empty())
            .unwrap_or(false);

    let content = match info.read().as_ref() {
        None => rsx! { div { class: "loading-spinner", "Loading…" } },
        Some(Err(_)) => rsx! { NotFound { username: plain_sig() } },
        Some(Ok(actor)) => rsx! {
            ProfileCard {
                actor: actor.clone(),
                is_own,
                is_logged_in,
                get_actor_posts_fn,
                get_actor_connections_fn,
                check_following_fn,
                update_profile_fn,
                upload_avatar_fn,
                follow_person_fn,
                unfollow_actor_fn,
                delete_fn,
                like_fn,
                unlike_fn,
                route_fn,
                update_fn,
            }
        },
    };

    // Always render AppShell — sidebar fills in reactively after hydration.
    rsx! {
    div { class: "page-content", {content} }
    }
}

#[component]
fn ProfileCard(
    actor: ActorInfo,
    is_own: bool,
    is_logged_in: bool,
    get_actor_posts_fn: GetActorPostsFn,
    get_actor_connections_fn: GetActorConnectionsFn,
    check_following_fn: CheckFollowingFn,
    update_profile_fn: UpdateProfileFn,
    upload_avatar_fn: UploadAvatarFn,
    follow_person_fn: FollowPersonFn,
    unfollow_actor_fn: UnfollowActorFn,
    delete_fn: DeleteObjectFn,
    like_fn: LikeFn,
    unlike_fn: LikeFn,
    route_fn: RouteFn,
    update_fn: UpdatePostFn,
) -> Element {
    let auth = use_context::<AuthSignal>();
    let token = auth
        .read()
        .as_ref()
        .map(|u| u.token.clone())
        .unwrap_or_default();

    let mut editing = use_signal(|| false);
    let mut display_name = use_signal(|| actor.display_name.clone().unwrap_or_default());
    let mut bio = use_signal(|| actor.bio.clone().unwrap_or_default());
    let mut saving = use_signal(|| false);
    let mut saved = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);
    let mut avatar_url = use_signal(|| actor.avatar_url.clone());
    let mut crop_modal: Signal<Option<CropModalState>> = use_signal(|| None);

    // Reset actor-derived signals when navigating to a different profile.
    // Keyed on username@domain so in-progress edits survive same-profile re-renders.
    let actor_key = format!("{}@{}", actor.username, actor.domain);
    let mut last_actor_key = use_signal(|| actor_key.clone());
    if *last_actor_key.peek() != actor_key {
        last_actor_key.set(actor_key);
        editing.set(false);
        display_name.set(actor.display_name.clone().unwrap_or_default());
        bio.set(actor.bio.clone().unwrap_or_default());
        avatar_url.set(actor.avatar_url.clone());
    }
    let mut photo_error = use_signal(|| Option::<String>::None);
    let mut connections_tab: Signal<Option<&'static str>> = use_signal(|| None);

    // Determine if the viewer may open the connections modal.
    // - Public profile or own profile: always yes.
    // - Private profile + logged in: only if viewer is an accepted follower.
    // - Private profile + not logged in: no.
    // Track ap_id + public_profile as signals so the resource re-runs on profile navigation.
    let mut actor_ap_id_sig = use_signal(|| actor.ap_id.clone());
    let mut actor_public_sig = use_signal(|| actor.public_profile);
    if *actor_ap_id_sig.peek() != actor.ap_id {
        actor_ap_id_sig.set(actor.ap_id.clone());
    }
    if *actor_public_sig.peek() != actor.public_profile {
        actor_public_sig.set(actor.public_profile);
    }
    let token_cv = token.clone();
    let can_view_connections = use_resource(move || {
        let own = is_own;
        let public = actor_public_sig();
        let token = token_cv.clone();
        let ap_id = actor_ap_id_sig();
        async move {
            if own || public {
                return true;
            }
            if token.is_empty() {
                return false;
            }
            check_following_fn
                .call(TokenApIdArgs { token, ap_id })
                .await
                .ok()
                .flatten()
                == Some(true)
        }
    });
    // For public / own profiles use true as the initial value to avoid a flash of
    // non-interactive stats while the resource resolves.
    let can_view = can_view_connections
        .read()
        .as_ref()
        .copied()
        .unwrap_or(is_own || actor.public_profile);

    // Fetch posts client-side; visibility depends on auth.
    // Peek-compare so use_resource re-runs when navigating to a different profile.
    let mut actor_username_sig = use_signal(|| actor.username.clone());
    if *actor_username_sig.peek() != actor.username {
        actor_username_sig.set(actor.username.clone());
    }
    let token_posts = token.clone();
    let posts = use_resource(move || {
        let token = if token_posts.is_empty() {
            None
        } else {
            Some(token_posts.clone())
        };
        async move {
            get_actor_posts_fn
                .call(GetActorPostsArgs {
                    username: actor_username_sig(),
                    token,
                })
                .await
        }
    });

    let follow_handle = format!("@{}@{}", actor.username, actor.domain);

    let on_save = {
        let token = token.clone();
        move |_: Event<MouseData>| {
            let token = token.clone();
            let display_name = display_name.peek().clone();
            let bio = bio.peek().clone();
            saving.set(true);
            saved.set(false);
            error.set(None);
            spawn(async move {
                let display_name = if display_name.trim().is_empty() {
                    None
                } else {
                    Some(display_name)
                };
                let bio = if bio.trim().is_empty() {
                    None
                } else {
                    Some(bio)
                };
                match update_profile_fn
                    .call(UpdateProfileArgs {
                        token,
                        display_name,
                        bio,
                    })
                    .await
                {
                    Ok(()) => {
                        saved.set(true);
                        editing.set(false);
                    }
                    Err(e) => error.set(Some(e)),
                }
                saving.set(false);
            });
        }
    };

    let on_avatar_change = move |_| {
        spawn(async move {
            photo_error.set(None);
            match prepare_selected_image_from_input("avatar-file-input").await {
                Ok(image) => crop_modal.set(Some(CropModalState {
                    object_url: image.object_url,
                    natural_width: image.natural_width,
                    natural_height: image.natural_height,
                    output_width: 400,
                    output_height: 400,
                    title: "Crop avatar".to_string(),
                    circle_mask: true,
                })),
                Err(err) => photo_error.set(Some(err)),
            }
        });
    };

    let on_crop_apply = {
        let token = token.clone();
        move |crop: CropSelection| {
            let token = token.clone();
            spawn(async move {
                photo_error.set(None);
                match compress_avatar_from_input("avatar-file-input", crop).await {
                    Ok(image) => match STANDARD.decode(&image.b64) {
                        Ok(bytes) => {
                            match upload_avatar_fn
                                .call(UploadAvatarArgs { token, bytes })
                                .await
                            {
                                Ok(url) => avatar_url.set(Some(url)),
                                Err(e) => photo_error.set(Some(e)),
                            }
                        }
                        Err(_) => photo_error.set(Some("Image encode failed".to_string())),
                    },
                    Err(err) => photo_error.set(Some(err)),
                }
                if let Some(current) = crop_modal.take() {
                    revoke_object_url(&current.object_url);
                }
            });
        }
    };

    let on_crop_cancel = move |_| {
        let _ = clear_file_input("avatar-file-input");
        if let Some(current) = crop_modal.take() {
            revoke_object_url(&current.object_url);
        }
    };

    let initial = actor
        .username
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".into());
    let display = if display_name.read().is_empty() {
        actor.username.clone()
    } else {
        display_name.read().clone()
    };

    rsx! {
        div { class: "card profile-card",
            div { class: "profile-header-row",
                div { class: Styles::avatar_upload_wrap,
                    div { class: "avatar-inner",
                        if let Some(url) = avatar_url.read().as_ref() {
                            img {
                                class: "avatar avatar-lg avatar-img",
                                src: "{url}",
                                alt: "{actor.username}",
                            }
                        } else {
                            div { class: "avatar avatar-lg", "{initial}" }
                        }
                        if is_own && *editing.read() {
                            label {
                                class: Styles::avatar_overlay_btn,
                                r#for: "avatar-file-input",
                                "📷"
                            }
                        }
                    }
                }
                if is_own {
                    input {
                        r#type: "file",
                        id: "avatar-file-input",
                        accept: "image/jpeg,image/png,image/webp",
                        style: "display:none",
                        onchange: on_avatar_change,
                    }
                }
                div { class: "profile-header-actions",
                    if is_own && !*editing.read() {
                        button {
                            class: format!("btn btn-ghost {}", Styles::profile_edit_btn),
                            "data-testid": "profile-edit-btn",
                            onclick: move |_| { editing.set(true); saved.set(false); },
                            "Edit profile"
                        }
                    }
                    // Renders nothing for is_own — FollowButton guards itself.
                    FollowButton {
                        is_own,
                        is_logged_in,
                        token: token.clone(),
                        ap_id: actor.ap_id.clone(),
                        handle: follow_handle.clone(),
                        check_following_fn,
                        follow_person_fn,
                        unfollow_actor_fn,
                    }
                }
            }

            div { class: "profile-body",
                if is_own {
                    if let Some(err) = photo_error.read().as_ref() {
                        ErrorBanner { message: err.clone() }
                    }
                }

                if is_own && *editing.read() {
                    div { class: Styles::profile_edit_form, "data-testid": "profile-edit-form",
                        div { class: "form-group",
                            label { "Display name" }
                            input {
                                r#type: "text",
                                placeholder: &*actor.username,
                                value: "{display_name}",
                                oninput: move |e| { display_name.set(e.value()); saved.set(false); },
                            }
                        }
                        div { class: "form-group",
                            label { "Bio" }
                            textarea {
                                placeholder: "Tell people about yourself…",
                                rows: "3",
                                value: "{bio}",
                                oninput: move |e| { bio.set(e.value()); saved.set(false); },
                            }
                        }
                        if let Some(err) = error.read().as_ref() {
                            ErrorBanner { message: err.clone() }
                        }
                        div { class: "settings-row",
                            button {
                                class: "btn btn-primary",
                                disabled: *saving.read(),
                                onclick: on_save,
                                if *saving.read() { "Saving…" } else { "Save" }
                            }
                            button {
                                class: "btn btn-ghost",
                                onclick: move |_| { editing.set(false); error.set(None); },
                                "Cancel"
                            }
                        }
                    }
                } else {
                    h2 { class: "profile-name", "{display}" }
                    p { class: "profile-handle", "@{actor.username}@{actor.domain}" }
                    if !bio.read().is_empty() {
                        p { class: Styles::profile_bio, "{bio}" }
                    }

                    div { class: "profile-stats",
                        if can_view {
                            button {
                                class: Styles::profile_stat,
                                "data-testid": "profile-stat",
                                onclick: move |_| { connections_tab.set(Some("following")); },
                                span { class: Styles::profile_stat_count, "{actor.following_count}" }
                                span { class: Styles::profile_stat_label, "Following" }
                            }
                        } else {
                            span { class: Styles::profile_stat, "data-testid": "profile-stat",
                                span { class: Styles::profile_stat_count, "{actor.following_count}" }
                                span { class: Styles::profile_stat_label, "Following" }
                            }
                        }
                        span { class: Styles::profile_stat_sep }
                        if can_view {
                            button {
                                class: Styles::profile_stat,
                                "data-testid": "profile-stat",
                                onclick: move |_| { connections_tab.set(Some("followers")); },
                                span { class: Styles::profile_stat_count, "{actor.followers_count}" }
                                span { class: Styles::profile_stat_label, "Followers" }
                            }
                        } else {
                            span { class: Styles::profile_stat, "data-testid": "profile-stat",
                                span { class: Styles::profile_stat_count, "{actor.followers_count}" }
                                span { class: Styles::profile_stat_label, "Followers" }
                            }
                        }
                    }

                    if is_own && *saved.read() {
                        span { class: Styles::saved_badge, "✓ Saved" }
                    }
                }

            }
        }

        match posts.read().as_ref() {
            None => rsx! { div { class: "loading-spinner", "Loading posts…" } },
            Some(Err(e)) => rsx! {
                div { class: Styles::profile_empty_posts, "data-testid": "profile-empty-posts", "{e}" }
            },
            Some(Ok(items)) if items.is_empty() => rsx! {
                if !actor.public_profile && !is_own {
                    div { class: format!("card {}", Styles::private_profile_notice), "data-testid": "private-profile-notice",
                        i { class: format!("ph ph-lock {}", Styles::private_profile_icon) }
                        div { class: Styles::private_profile_text,
                            strong { "This profile is private." }
                            span { " Follow to see their posts." }
                        }
                    }
                } else {
                    div { class: Styles::profile_empty_posts, "data-testid": "profile-empty-posts", "No posts yet." }
                }
            },
            Some(Ok(items)) => rsx! {
                {items.iter().map(|item| {
                    let tok = if token.is_empty() { None } else { Some(token.clone()) };
                    rsx! {
                        FeedCard {
                            key: "{item.id}",
                            item: item.clone(),
                            token: tok,
                            on_deleted: {
                                let mut posts = posts;
                                move |_| posts.restart()
                            },
                            on_edited: {
                                let mut posts = posts;
                                move |_| posts.restart()
                            },
                            delete_fn,
                            like_fn,
                            unlike_fn,
                            route_fn,
                            update_fn,
                        }
                    }
                })}
            },
        }

        if connections_tab().is_some() {
            ConnectionsModal {
                username: actor.username.clone(),
                token: if token.is_empty() { None } else { Some(token.clone()) },
                connections_tab,
                get_actor_connections_fn,
            }
        }

        if let Some(crop_state) = crop_modal() {
            CropModal {
                state: crop_state,
                on_cancel: on_crop_cancel,
                on_apply: on_crop_apply,
            }
        }
    }
}

#[component]
fn ConnectionsModal(
    username: String,
    token: Option<String>,
    mut connections_tab: Signal<Option<&'static str>>,
    get_actor_connections_fn: GetActorConnectionsFn,
) -> Element {
    let nav = use_navigator();
    let mut fetched: Signal<Option<ConnectionsResult>> = use_signal(|| None);
    let mut loading = use_signal(|| false);

    // Fetch once when the modal first opens; skip on subsequent tab switches.
    use_effect(move || {
        let tab = connections_tab();
        if tab.is_some() && fetched.read().is_none() && !*loading.read() {
            loading.set(true);
            let username = username.clone();
            let token = token.clone();
            spawn(async move {
                if let Ok(result) = get_actor_connections_fn
                    .call(GetActorConnectionsArgs { username, token })
                    .await
                {
                    fetched.set(Some(result));
                }
                loading.set(false);
            });
        }
    });

    let active_tab = connections_tab().unwrap_or("following");

    // Clone data out of the signal so we can use it in RSX without a live borrow.
    let data: Option<ConnectionsResult> = fetched.read().clone();

    rsx! {
        div {
            class: "modal-backdrop",
            onclick: move |_| { connections_tab.set(None); },

            div {
                class: "modal-card connections-modal",
                onclick: move |e| { e.stop_propagation(); },

                div { class: "modal-header",
                    div { class: "modal-tabs",
                        button {
                            class: if active_tab == "following" { "modal-tab modal-tab-active" } else { "modal-tab" },
                            onclick: move |_| { connections_tab.set(Some("following")); },
                            "Following"
                        }
                        button {
                            class: if active_tab == "followers" { "modal-tab modal-tab-active" } else { "modal-tab" },
                            onclick: move |_| { connections_tab.set(Some("followers")); },
                            "Followers"
                        }
                    }
                    button {
                        class: "modal-close",
                        onclick: move |_| { connections_tab.set(None); },
                        "×"
                    }
                }

                div { class: "modal-body",
                    if *loading.read() || data.is_none() {
                        div { class: "loading-spinner", "Loading…" }
                    } else {
                        match data {
                            None => rsx! {},
                            Some(ref result) => {
                                let items: &[ConnectionItem] = if active_tab == "following" {
                                    &result.following
                                } else {
                                    &result.followers
                                };
                                if items.is_empty() {
                                    rsx! {
                                        div { class: "connections-empty",
                                            if active_tab == "following" { "Not following anyone yet." }
                                            else { "No followers yet." }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        {items.iter().map(|item| {
                                            let nav2 = nav;
                                            let uname = item.username.clone();
                                            let is_local = item.is_local;
                                            let domain = item.domain.clone();
                                            let handle = if is_local {
                                                format!("@{uname}")
                                            } else {
                                                format!("@{uname}@{domain}")
                                            };
                                            let display_name = item.display_name
                                                .clone()
                                                .unwrap_or_else(|| item.username.clone());
                                            // Only local profiles are navigable; remote ones
                                            // are not hosted here so routing would 404.
                                            let nav_handle = if is_local {
                                                Some(format!("@{uname}"))
                                            } else {
                                                None
                                            };
                                            rsx! {
                                                div {
                                                    key: "{item.ap_id}",
                                                    class: if nav_handle.is_some() { "connection-row connection-row-link" } else { "connection-row" },
                                                    onclick: move |_| {
                                                        if let Some(ref h) = nav_handle {
                                                            connections_tab.set(None);
                                                            nav2.push(format!("/{h}"));
                                                        }
                                                    },
                                                    Avatar { size: AvatarSize::Small, url: None, name: display_name.clone() },
                                                    div { class: "connection-info",
                                                        span { class: "connection-name", "{display_name}" }
                                                        span { class: "connection-handle", "{handle}" }
                                                    }
                                                }
                                            }
                                        })}
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FollowButton(
    is_own: bool,
    is_logged_in: bool,
    token: String,
    ap_id: String,
    handle: String,
    check_following_fn: CheckFollowingFn,
    follow_person_fn: FollowPersonFn,
    unfollow_actor_fn: UnfollowActorFn,
) -> Element {
    // None = not following, Some(false) = pending, Some(true) = accepted
    let mut follow_status = use_signal(|| Option::<Option<bool>>::None);
    let mut in_flight = use_signal(|| false);
    let mut follow_error = use_signal(|| Option::<String>::None);
    let mut copied = use_signal(|| false);

    // Peek-compare: sync props into signals so use_resource / RSX re-run on navigation.
    let mut is_own_sig = use_signal(|| is_own);
    let mut is_logged_in_sig = use_signal(|| is_logged_in);
    let mut ap_id_sig = use_signal(|| ap_id.clone());

    if *is_own_sig.peek() != is_own {
        is_own_sig.set(is_own);
    }
    if *is_logged_in_sig.peek() != is_logged_in {
        is_logged_in_sig.set(is_logged_in);
    }
    if *ap_id_sig.peek() != ap_id {
        ap_id_sig.set(ap_id.clone());
    }

    // Fetch follow status. Re-runs automatically on profile navigation (ap_id change).
    let token_check = token.clone();
    let mut follow_resource = use_resource(move || {
        let logged_in = is_logged_in_sig();
        let own = is_own_sig();
        let token = token_check.clone();
        let ap_id = ap_id_sig();
        async move {
            if logged_in && !own {
                check_following_fn
                    .call(TokenApIdArgs { token, ap_id })
                    .await
                    .ok()
            } else {
                None
            }
        }
    });

    // Sync resource into follow_status only when no mutation is in flight.
    // When in_flight is true the optimistic value owns follow_status.
    // When the resource is restarting (outer None = loading) we don't override.
    if !*in_flight.peek() {
        if let Some(resource_val) = follow_resource.read().as_ref() {
            // resource_val: &Option<Option<bool>> — inner None = not following,
            // Some(false) = pending, Some(true) = accepted.
            if *follow_status.peek() != *resource_val {
                follow_status.set(*resource_val);
            }
        }
    }

    let token_follow = token.clone();
    let ap_id_follow = ap_id.clone();
    let on_follow = move |_: Event<MouseData>| {
        let token = token_follow.clone();
        let handle_or_url = ap_id_follow.clone();
        in_flight.set(true);
        follow_error.set(None);
        spawn(async move {
            match follow_person_fn
                .call(FollowArgs {
                    token,
                    handle_or_url,
                })
                .await
            {
                Ok(()) => {
                    // Optimistic: show pending (Some(false)) — server will confirm
                    follow_status.set(Some(Some(false)));
                    follow_resource.restart();
                }
                Err(e) => follow_error.set(Some(e)),
            }
            in_flight.set(false);
        });
    };

    let token_unfollow = token.clone();
    let ap_id_unfollow = ap_id.clone();
    let on_unfollow = move |_: Event<MouseData>| {
        let token = token_unfollow.clone();
        let ap_id = ap_id_unfollow.clone();
        in_flight.set(true);
        follow_error.set(None);
        spawn(async move {
            match unfollow_actor_fn.call(TokenApIdArgs { token, ap_id }).await {
                Ok(()) => {
                    follow_status.set(Some(None));
                    follow_resource.restart();
                }
                Err(e) => follow_error.set(Some(e)),
            }
            in_flight.set(false);
        });
    };

    let copy_handle = handle.clone();
    let on_copy_handle = move |_: Event<MouseData>| {
        let h = copy_handle.clone();
        spawn(async move {
            if copy_to_clipboard(&h).await.is_ok() {
                copied.set(true);
                sleep_ms(1_500).await;
                copied.set(false);
            }
        });
    };

    rsx! {
        if !*is_own_sig.read() {
            if *is_logged_in_sig.read() {
                match *follow_status.read() {
                    None => rsx! {
                        button { class: "btn btn-ghost", disabled: true, "…" }
                    },
                    Some(None) => rsx! {
                        button {
                            class: "btn btn-primary",
                            disabled: *in_flight.read(),
                            onclick: on_follow,
                            if *in_flight.read() { "Following…" } else { "Follow" }
                        }
                    },
                    Some(Some(false)) => rsx! {
                        button {
                            class: format!("btn btn-ghost {}", Styles::follow_pending_btn),
                            disabled: *in_flight.read(),
                            onclick: on_unfollow,
                            if *in_flight.read() { "…" } else { "Pending…" }
                        }
                    },
                    Some(Some(true)) => rsx! {
                        button {
                            class: "btn btn-ghost",
                            disabled: *in_flight.read(),
                            onclick: on_unfollow,
                            if *in_flight.read() { "Unfollowing…" } else { "Unfollow" }
                        }
                    },
                }
            } else {
                button {
                    class: if *copied.read() { "btn btn-ghost" } else { "btn btn-primary" },
                    onclick: on_copy_handle,
                    if *copied.read() { "✓ Copied!" } else { "Follow" }
                }
            }
            if let Some(err) = follow_error.read().clone() {
                ErrorBanner { message: err }
            }
        }
    }
}

#[component]
fn NotFound(username: String) -> Element {
    let nav = use_navigator();
    rsx! {
        div { class: Styles::not_found_page,
            div { class: Styles::not_found_blob_wrap,
                div { class: format!("{} {}", Styles::nf_blob, Styles::nf_blob_a) }
                div { class: format!("{} {}", Styles::nf_blob, Styles::nf_blob_b) }
            }
            div { class: "not-found-card",
                div { class: Styles::nf_illustration,
                    i { class: format!("ph ph-person-simple-run {}", Styles::nf_runner) }
                    span { class: "nf-arrow", "←" }
                    i { class: "ph ph-flag-checkered nf-flag" }
                }
                p { class: Styles::nf_label, "not here" }
                h1 { class: Styles::nf_title, "DNF" }
                p { class: Styles::nf_handle, "@{username}" }
                p { class: Styles::nf_desc, "This is a single-user server." }
                p { class: Styles::nf_hint,
                    "You're looking for someone on a different server. Search for "
                    code { "@{username}" }
                    " there."
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| { nav.push("/"); },
                    "Go to home"
                }
            }
        }
    }
}

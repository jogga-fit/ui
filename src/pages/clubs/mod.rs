use dioxus::prelude::*;

use crate::{
    DeleteObjectFn, FollowActorFn, FollowArgs, GetClubFeedArgs, GetClubFeedFn, LikeFn, RouteFn,
    TokenApIdArgs, UnfollowActorFn, UpdatePostFn,
    components::{
        empty_state::EmptyState, error_banner::ErrorBanner, post::FeedCard,
        remote_follow_card::RemoteFollowCard,
    },
    types::{FollowingItem, sleep_ms},
};

#[css_module("/src/pages/clubs/style.css")]
struct Styles;

#[component]
pub fn ClubsPageView(
    token: String,
    clubs: Option<Result<Vec<FollowingItem>, String>>,
    on_clubs_refresh: EventHandler<()>,
    follow_actor_fn: FollowActorFn,
    unfollow_actor_fn: UnfollowActorFn,
    #[props(default = true)] is_logged_in: bool,
) -> Element {
    rsx! {
        div { class: "page-content",
            div { class: Styles::clubs_header,
                h1 { class: "settings-title", "Clubs" }
            }

            if is_logged_in {
                FindClubCard {
                    token: token.clone(),
                    on_joined: move |_| on_clubs_refresh.call(()),
                    follow_actor_fn,
                }
            }

            div { "data-testid": "clubs",
                match clubs {
                    None => rsx! { div { class: "loading-spinner", "Loading clubs…" } },
                    Some(Err(e)) => rsx! { ErrorBanner { message: e } },
                    Some(Ok(items)) if items.is_empty() => rsx! {
                        EmptyState {
                            icon: "ph ph-users-three".to_string(),
                            title: "No clubs yet".to_string(),
                            p { "Join remote clubs by handle using the form above." }
                        }
                    },
                    Some(Ok(items)) => rsx! {
                        div { class: Styles::clubs_grid,
                            {items.iter().map(|club| {
                                rsx! {
                                    ClubCard {
                                        key: "{club.ap_id}",
                                        club: club.clone(),
                                        token: token.clone(),
                                        on_change: move |_| on_clubs_refresh.call(()),
                                        unfollow_actor_fn,
                                    }
                                }
                            })}
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn FindClubCard(
    token: String,
    on_joined: EventHandler<()>,
    follow_actor_fn: FollowActorFn,
) -> Element {
    let mut target = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);
    let mut done = use_signal(|| false);

    let on_join = move |_: ()| {
        let token = token.clone();
        let handle_or_url = target.read().trim().to_string();
        if handle_or_url.is_empty() {
            return;
        }
        busy.set(true);
        error.set(None);
        done.set(false);
        spawn(async move {
            match follow_actor_fn
                .call(FollowArgs {
                    token,
                    handle_or_url,
                })
                .await
            {
                Ok(()) => {
                    target.set(String::new());
                    done.set(true);
                    sleep_ms(1_200).await;
                    done.set(false);
                    on_joined.call(());
                }
                Err(e) => error.set(Some(e)),
            }
            busy.set(false);
        });
    };

    rsx! {
        RemoteFollowCard {
            icon_class: "ph ph-users-three".to_string(),
            title: "Find a club on another instance".to_string(),
            description: "Enter a federated club handle to request membership and receive posts in your local feed.".to_string(),
            input_label: "Club handle".to_string(),
            placeholder: "@clubname@instance.example".to_string(),
            value: target.read().clone(),
            examples: vec!["@sfcycling@jogga.fit".to_string(), "@trailcrew@example.social".to_string()],
            button_icon: "ph ph-arrow-square-in".to_string(),
            button_label: "Find & join".to_string(),
            busy_label: "Joining".to_string(),
            busy: *busy.read(),
            disabled: target.read().trim().is_empty(),
            error: error.read().clone(),
            success_message: if *done.read() { Some("Join request sent".to_string()) } else { None },
            on_input: move |value: String| {
                target.set(value);
                done.set(false);
                error.set(None);
            },
            on_submit: on_join,
        }
    }
}

#[component]
fn ClubCard(
    club: FollowingItem,
    token: String,
    on_change: EventHandler<()>,
    unfollow_actor_fn: UnfollowActorFn,
) -> Element {
    let nav = use_navigator();
    let handle = club.username.clone();
    let domain = club.domain.clone();
    let display = club
        .display_name
        .as_deref()
        .unwrap_or(&club.username)
        .to_string();
    let initial = display
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".into());

    let handle_at = if domain.is_empty() {
        format!("@{handle}")
    } else {
        format!("@{handle}@{domain}")
    };
    let remote_club_url = if domain.is_empty() {
        None
    } else {
        Some(format!("https://{domain}/clubs/{handle}"))
    };

    let ap_id = club.ap_id.clone();
    let route_handle = format!("{handle}@{domain}");

    rsx! {
        div { class: format!("card {}", Styles::club_card),
            div { class: Styles::club_card_header,
                div { class: format!("avatar avatar-md {}", Styles::club_avatar), "{initial}" }
                div { class: Styles::club_card_info,
                    button {
                        class: Styles::club_name_btn,
                        onclick: move |_| { nav.push(format!("/clubs/{route_handle}")); },
                        "{display}"
                    }
                    div { class: Styles::club_meta,
                        if let Some(url) = remote_club_url {
                            a {
                                class: "connection-handle",
                                href: "{url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "{handle_at}"
                            }
                        } else {
                            span { class: "connection-handle", "{handle_at}" }
                        }
                        if club.accepted {
                            span { class: format!("{} {}", Styles::club_badge, Styles::club_badge_open), "Member" }
                        } else {
                            span { class: format!("{} {}", Styles::club_badge, Styles::club_badge_exclusive), "Pending" }
                        }
                    }
                }
            }

            div { class: Styles::club_card_actions,
                LeaveButton {
                    token: token.clone(),
                    ap_id: ap_id.clone(),
                    accepted: club.accepted,
                    on_change: move |_| on_change.call(()),
                    unfollow_actor_fn,
                }
            }
        }
    }
}

#[component]
fn LeaveButton(
    token: String,
    ap_id: String,
    accepted: bool,
    on_change: EventHandler<()>,
    unfollow_actor_fn: UnfollowActorFn,
) -> Element {
    let mut busy = use_signal(|| false);
    let mut confirming = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    let leave_label = if accepted { "Leave" } else { "Cancel request" };
    let busy_label = if accepted {
        "Leaving…"
    } else {
        "Cancelling…"
    };
    let role_label = if accepted { "Member" } else { "Pending" };

    rsx! {
        div { class: Styles::club_leave_group,
            if *confirming.read() {
                div { class: "unfollow-confirm",
                    span { class: "unfollow-confirm-text", "{leave_label}?" }
                    button {
                        class: "btn btn-sm btn-reject",
                        disabled: *busy.read(),
                        onclick: move |_| {
                            let token = token.clone();
                            let ap_id = ap_id.clone();
                            busy.set(true);
                            error.set(None);
                            spawn(async move {
                                match unfollow_actor_fn.call(TokenApIdArgs { token, ap_id }).await {
                                    Ok(()) => on_change.call(()),
                                    Err(e) => error.set(Some(e)),
                                }
                                busy.set(false);
                                confirming.set(false);
                            });
                        },
                        if *busy.read() { "{busy_label}" } else { "Yes, leave" }
                    }
                    button {
                        class: "btn btn-sm btn-ghost",
                        onclick: move |_| confirming.set(false),
                        "Cancel"
                    }
                }
            } else {
                button {
                    class: format!("btn btn-ghost btn-sm {}", Styles::club_leave_btn),
                    onclick: move |_| confirming.set(true),
                    span { "{role_label}" }
                    i { class: format!("ph ph-x {}", Styles::club_leave_icon) }
                }
            }
            if let Some(err) = error.read().as_ref() {
                span { class: Styles::club_join_error, "{err}" }
            }
        }
    }
}

#[component]
pub fn ClubDetailPageView(
    handle: String,
    club: Option<Result<Option<FollowingItem>, String>>,
    token: String,
    on_back: EventHandler<()>,
    on_left: EventHandler<()>,
    get_club_feed_fn: GetClubFeedFn,
    unfollow_actor_fn: UnfollowActorFn,
    delete_fn: DeleteObjectFn,
    like_fn: LikeFn,
    unlike_fn: LikeFn,
    route_fn: RouteFn,
    update_fn: UpdatePostFn,
) -> Element {
    rsx! {
        div { class: "page-content",
            div { class: Styles::breadcrumb,
                button {
                    class: Styles::breadcrumb_back,
                    onclick: move |_| on_back.call(()),
                    i { class: "ph ph-arrow-left" }
                    " Clubs"
                }
            }
            match club {
                None => rsx! { div { class: "loading-spinner", "Loading…" } },
                Some(Err(e)) => rsx! { ErrorBanner { message: e } },
                Some(Ok(None)) => rsx! {
                    div { class: "not-found-card",
                        h1 { "Not a member" }
                        p { "You have not joined @{handle}." }
                        p {
                            "Use the "
                            button {
                                class: "link-btn",
                                onclick: move |_| on_back.call(()),
                                "Clubs"
                            }
                            " page to find and join clubs."
                        }
                    }
                },
                Some(Ok(Some(club))) => rsx! {
                    ClubDetailCard {
                        club: club.clone(),
                        token: token.clone(),
                        on_change: move |_| on_left.call(()),
                        unfollow_actor_fn,
                    }
                    ClubFeed {
                        token: token.clone(),
                        club: club.clone(),
                        get_club_feed_fn,
                        delete_fn,
                        like_fn,
                        unlike_fn,
                        route_fn,
                        update_fn,
                    }
                },
            }
        }
    }
}

#[component]
fn ClubFeed(
    token: String,
    club: FollowingItem,
    get_club_feed_fn: GetClubFeedFn,
    delete_fn: DeleteObjectFn,
    like_fn: LikeFn,
    unlike_fn: LikeFn,
    route_fn: RouteFn,
    update_fn: UpdatePostFn,
) -> Element {
    let handle = format!("{}@{}", club.username, club.domain);
    let token_clone = token.clone();
    let posts = use_resource(move || {
        let t = token_clone.clone();
        let h = handle.clone();
        async move {
            get_club_feed_fn
                .call(GetClubFeedArgs {
                    handle: h,
                    token: Some(t),
                })
                .await
        }
    });

    let remote_url = if !club.domain.is_empty() {
        Some(format!("https://{}/clubs/{}", club.domain, club.username))
    } else {
        None
    };

    rsx! {
        div { class: Styles::club_feed,
            match posts.read().as_ref() {
                None => rsx! { div { class: "loading-spinner", "Loading posts…" } },
                Some(Err(e)) => rsx! { ErrorBanner { message: e.clone() } },
                Some(Ok(items)) if items.is_empty() => rsx! {
                    div { class: "empty-state",
                        p { "No posts received from this club yet." }
                    }
                },
                Some(Ok(items)) => rsx! {
                    {items.iter().map(|item| rsx! {
                        FeedCard {
                            key: "{item.id}",
                            item: item.clone(),
                            token: Some(token.clone()),
                            delete_fn,
                            like_fn,
                            unlike_fn,
                            route_fn,
                            update_fn,
                        }
                    })}
                },
            }
            if let Some(url) = remote_url {
                if posts.read().as_ref().is_some() {
                    p { class: Styles::club_feed_remote_notice,
                        "Showing only posts received by this server. "
                        a {
                            href: "{url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "View all posts on {club.domain}"
                            i { class: "ph ph-arrow-square-out", style: "margin-left: 4px; font-size: 0.85em;" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ClubDetailCard(
    club: FollowingItem,
    token: String,
    on_change: EventHandler<()>,
    unfollow_actor_fn: UnfollowActorFn,
) -> Element {
    let display = club
        .display_name
        .as_deref()
        .unwrap_or(&club.username)
        .to_string();
    let initial = display
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".into());
    let handle_at = if club.domain.is_empty() {
        format!("@{}", club.username)
    } else {
        format!("@{}@{}", club.username, club.domain)
    };
    let remote_club_url = if club.domain.is_empty() {
        None
    } else {
        Some(format!("https://{}/clubs/{}", club.domain, club.username))
    };

    rsx! {
        div { class: "card profile-card",
            div { class: "profile-header-row",
                div { class: format!("avatar avatar-lg {}", Styles::club_avatar), "{initial}" }
                div { class: "profile-header-actions",
                    LeaveButton {
                        token: token.clone(),
                        ap_id: club.ap_id.clone(),
                        accepted: club.accepted,
                        on_change: move |_| on_change.call(()),
                        unfollow_actor_fn,
                    }
                }
            }
            div { class: "profile-body",
                h2 { class: "profile-name", "{display}" }
                if let Some(url) = remote_club_url {
                    a {
                        class: "profile-handle",
                        href: "{url}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "{handle_at}"
                    }
                } else {
                    p { class: "profile-handle", "{handle_at}" }
                }
                div { class: "profile-stats",
                    if club.accepted {
                        span { class: format!("{} {}", Styles::club_badge, Styles::club_badge_open), "Member" }
                    } else {
                        span { class: format!("{} {}", Styles::club_badge, Styles::club_badge_exclusive), "Pending approval" }
                    }
                }
            }
        }
    }
}

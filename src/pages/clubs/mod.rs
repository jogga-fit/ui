use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    DeleteObjectFn, FollowActorFn, FollowArgs, GetClubFeedArgs, GetClubFeedFn, LikeFn, RouteFn,
    TokenApIdArgs, UnfollowActorFn, UpdatePostFn,
    browser::copy_to_clipboard,
    components::{
        empty_state::EmptyState, error_banner::ErrorBanner, post::FeedCard,
        remote_follow_card::RemoteFollowCard,
    },
    sleep_ms,
    types::FollowingItem,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ClubRole {
    Member,
    Moderator,
    Admin,
    NotMember,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClubItem {
    pub handle: String,
    pub ap_id: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub exclusive: bool,
    pub member_count: i64,
    pub my_role: ClubRole,
}

/// Lightweight club summary used on profile pages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClubSummary {
    pub handle: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClubMemberItem {
    pub ap_id: String,
    pub username: String,
    pub domain: String,
    #[serde(default)]
    pub is_local: bool,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    /// `None` = plain member, `"moderator"`, `"admin"`.
    pub role: Option<ClubRole>,
    pub accepted: bool,
}

#[css_module("/src/pages/clubs/style.css")]
struct Styles;

fn initial_char(name: &str) -> String {
    name.chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".into())
}

#[component]
pub fn ClubsPageView(
    token: String,
    clubs: Option<Result<Vec<FollowingItem>, String>>,
    on_clubs_refresh: EventHandler<()>,
    follow_actor_fn: FollowActorFn,
    unfollow_actor_fn: UnfollowActorFn,
    #[props(default = true)] is_logged_in: bool,
    #[props(default)] server_clubs: Vec<ClubItem>,
) -> Element {
    let joined_count = clubs
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .map(|v| v.len());
    let server_count = server_clubs.len();

    rsx! {
        div { class: "page-content",
            div { class: Styles::clubs_header,
                div { class: Styles::clubs_header_row,
                    h1 { class: "settings-title", "Clubs" }
                    if !is_logged_in && server_count > 0 {
                        span { class: Styles::clubs_count_badge, "{server_count}" }
                    }
                    if is_logged_in {
                        if let Some(n) = joined_count {
                            if n > 0 {
                                span { class: Styles::clubs_count_badge, "{n} joined" }
                            }
                        }
                    }
                }
            }

            if is_logged_in {
                FindClubCard {
                    token: token.clone(),
                    on_joined: move |_| on_clubs_refresh.call(()),
                    follow_actor_fn,
                }

                div { "data-testid": "clubs",
                    match clubs {
                        None => rsx! { div { class: "loading-spinner", "Loading clubs…" } },
                        Some(Err(ref e)) => rsx! { ErrorBanner { message: e.clone() } },
                        Some(Ok(ref items)) if items.is_empty() => rsx! {},
                        Some(Ok(ref items)) => rsx! {
                            p { class: Styles::clubs_section_label,
                                if server_count > 0 { "Joined" } else { "Your clubs" }
                            }
                            div { class: "card",
                                {items.iter().map(|club| {
                                    let display = club.display_name.as_deref().unwrap_or(&club.username).to_string();
                                    let handle = club.username.clone();
                                    let domain = club.domain.clone();
                                    let handle_at = if domain.is_empty() {
                                        format!("@{handle}")
                                    } else {
                                        format!("@{handle}@{domain}")
                                    };
                                    let route_handle = format!("{handle}@{domain}");
                                    rsx! {
                                        JoinedClubRow {
                                            key: "{club.ap_id}",
                                            display,
                                            handle_at,
                                            route: route_handle,
                                            token: token.clone(),
                                            ap_id: club.ap_id.clone(),
                                            accepted: club.accepted,
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

            if server_count > 0 {
                if is_logged_in {
                    div { class: Styles::clubs_section_divider }
                    p { class: Styles::clubs_section_label, "On this server" }
                }
                div { class: "card",
                    {server_clubs.iter().map(|club| {
                        let display = club.display_name.as_deref().unwrap_or(&club.handle).to_string();
                        let handle_at = format!("@{}", club.handle);
                        let route_handle = club.handle.clone();
                        rsx! {
                            ServerClubRow {
                                key: "{club.ap_id}",
                                display,
                                handle_at,
                                route: route_handle,
                                exclusive: club.exclusive,
                                member_count: club.member_count,
                                my_role: club.my_role,
                                ap_id: club.ap_id.clone(),
                                is_logged_in,
                                token: token.clone(),
                                on_joined: move |_| on_clubs_refresh.call(()),
                                follow_actor_fn,
                            }
                        }
                    })}
                }
            } else if !is_logged_in {
                EmptyState {
                    icon: "ph ph-users-three".to_string(),
                    title: "No clubs yet".to_string(),
                    p { "No clubs have been created on this server yet." }
                }
            } else if matches!(&clubs, Some(Ok(items)) if items.is_empty()) {
                EmptyState {
                    icon: "ph ph-users-three".to_string(),
                    title: "No clubs yet".to_string(),
                    p { "Join remote clubs by handle using the form above." }
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

/// Shared row shell: square avatar + name/handle on left, children on right.
#[component]
fn ClubRow(
    display: String,
    initial: String,
    handle_at: String,
    route: String,
    children: Element,
) -> Element {
    let nav = use_navigator();
    rsx! {
        div { class: Styles::club_row,
            div {
                class: Styles::club_row_identity,
                onclick: move |_| { nav.push(format!("/clubs/{route}")); },
                div { class: "avatar avatar-sm {Styles::club_avatar_row}", "{initial}" }
                div { class: Styles::club_row_info,
                    span { class: Styles::club_row_name, "{display}" }
                    span { class: Styles::club_row_handle, "{handle_at}" }
                }
            }
            div { class: Styles::club_row_actions,
                {children}
            }
        }
    }
}

#[component]
fn JoinedClubRow(
    display: String,
    handle_at: String,
    route: String,
    token: String,
    ap_id: String,
    accepted: bool,
    on_change: EventHandler<()>,
    unfollow_actor_fn: UnfollowActorFn,
) -> Element {
    let initial = initial_char(&display);
    let mut busy = use_signal(|| false);
    let mut confirming = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);
    let leave_label = if accepted { "Leave" } else { "Cancel" };
    let busy_label = if accepted { "Leaving…" } else { "Cancelling…" };

    rsx! {
        ClubRow { display, initial, handle_at, route,
            if *confirming.read() {
                div { class: Styles::club_leave_confirm,
                    span { class: Styles::club_leave_confirm_text, "{leave_label}?" }
                    button {
                        class: Styles::club_confirm_btn,
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
                        if *busy.read() { "{busy_label}" } else { "Leave" }
                    }
                    button {
                        class: Styles::club_cancel_btn,
                        onclick: move |_| confirming.set(false),
                        "Keep"
                    }
                }
            } else {
                if accepted {
                    span { class: "{Styles::club_badge} {Styles::club_badge_open}", "Member" }
                } else {
                    span { class: "{Styles::club_badge} {Styles::club_badge_exclusive}", "Pending" }
                }
                button {
                    class: "btn btn-ghost btn-sm {Styles::club_leave_btn}",
                    onclick: move |_| confirming.set(true),
                    i { class: "ph ph-x {Styles::club_leave_icon}" }
                }
            }
            if let Some(err) = error.read().as_ref() {
                span { class: Styles::club_join_error, "{err}" }
            }
        }
    }
}

#[component]
fn ServerClubRow(
    display: String,
    handle_at: String,
    route: String,
    exclusive: bool,
    member_count: i64,
    my_role: ClubRole,
    ap_id: String,
    is_logged_in: bool,
    token: String,
    on_joined: EventHandler<()>,
    follow_actor_fn: FollowActorFn,
) -> Element {
    let initial = initial_char(&display);
    let mut busy = use_signal(|| false);
    let mut join_error = use_signal(|| Option::<String>::None);
    let mut request_sent = use_signal(|| false);

    let already_member = !matches!(my_role, ClubRole::NotMember);
    let post_join_label = if exclusive { "Requested" } else { "Joined" };

    rsx! {
        ClubRow { display, initial, handle_at, route,
            span { class: Styles::club_member_count,
                i { class: "ph ph-users" }
                " {member_count}"
            }
            if exclusive {
                span { class: "{Styles::club_badge} {Styles::club_badge_exclusive}", "Exclusive" }
            } else {
                span { class: "{Styles::club_badge} {Styles::club_badge_open}", "Open" }
            }
            if is_logged_in {
                if already_member {
                    span {
                        class: "{Styles::club_badge} {Styles::club_badge_open}",
                        match my_role {
                            ClubRole::Admin => "Admin",
                            ClubRole::Moderator => "Mod",
                            _ => "Member",
                        }
                    }
                } else if *request_sent.read() {
                    span { class: "{Styles::club_badge} {Styles::club_badge_exclusive}", "{post_join_label}" }
                } else {
                    button {
                        class: "btn btn-sm btn-primary",
                        disabled: *busy.read(),
                        onclick: move |_| {
                            let t = token.clone();
                            let aid = ap_id.clone();
                            busy.set(true);
                            join_error.set(None);
                            spawn(async move {
                                match follow_actor_fn.call(FollowArgs {
                                    token: t,
                                    handle_or_url: aid,
                                })
                                .await
                                {
                                    Ok(()) => {
                                        request_sent.set(true);
                                        on_joined.call(());
                                    }
                                    Err(e) => join_error.set(Some(e)),
                                }
                                busy.set(false);
                            });
                        },
                        if *busy.read() { "Joining…" } else { "Join" }
                    }
                }
            }
            if let Some(err) = join_error.read().as_ref() {
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
    #[props(default)] description: Option<String>,
    #[props(default = true)] is_logged_in: bool,
    #[props(default)] server_club: Option<ClubItem>,
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
                    if let Some(club) = server_club {
                        ClubPublicCard {
                            club,
                            description: description.clone(),
                        }
                    } else {
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
                    }
                },
                Some(Ok(Some(club))) => rsx! {
                    ClubDetailCard {
                        club: club.clone(),
                        token: token.clone(),
                        on_change: move |_| on_left.call(()),
                        unfollow_actor_fn,
                        description: description.clone(),
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
        let token = Some(token_clone.clone());
        let handle = handle.clone();
        async move {
            get_club_feed_fn
                .call(GetClubFeedArgs { handle, token })
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

/// Shared profile card shell for club detail views.
#[component]
fn ClubProfileCard(
    display: String,
    initial: String,
    handle_at: String,
    #[props(default)] remote_url: Option<String>,
    #[props(default)] description: Option<String>,
    stats: Element,
    children: Element,
) -> Element {
    rsx! {
        div { class: "card profile-card",
            div { class: Styles::club_profile_cover }
            div { class: "profile-header-row",
                div { class: "avatar avatar-lg {Styles::club_avatar}", "{initial}" }
                div { class: "profile-header-actions",
                    {children}
                }
            }
            div { class: "profile-body",
                h2 { class: "profile-name", "{display}" }
                if let Some(url) = remote_url {
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
                    {stats}
                }
                if let Some(desc) = description {
                    p { class: Styles::club_about_text, "{desc}" }
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
    #[props(default)] description: Option<String>,
) -> Element {
    let display = club
        .display_name
        .as_deref()
        .unwrap_or(&club.username)
        .to_string();
    let initial = initial_char(&display);
    let handle_at = if club.domain.is_empty() {
        format!("@{}", club.username)
    } else {
        format!("@{}@{}", club.username, club.domain)
    };
    let remote_url = if club.domain.is_empty() {
        None
    } else {
        Some(format!("https://{}/clubs/{}", club.domain, club.username))
    };
    let leave_label = if club.accepted { "Leave" } else { "Cancel request" };
    let busy_label = if club.accepted { "Leaving…" } else { "Cancelling…" };
    let ap_id = club.ap_id.clone();
    let mut busy = use_signal(|| false);
    let mut confirming = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    rsx! {
        ClubProfileCard {
            display,
            initial,
            handle_at,
            remote_url,
            description,
            stats: rsx! {
                if club.accepted {
                    span { class: "{Styles::club_badge} {Styles::club_badge_open}", "Member" }
                } else {
                    span { class: "{Styles::club_badge} {Styles::club_badge_exclusive}", "Pending approval" }
                }
            },
            if *confirming.read() {
                div { class: Styles::club_leave_confirm,
                    span { class: Styles::club_leave_confirm_text, "{leave_label}?" }
                    button {
                        class: Styles::club_confirm_btn,
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
                        if *busy.read() { "{busy_label}" } else { "Leave" }
                    }
                    button {
                        class: Styles::club_cancel_btn,
                        onclick: move |_| confirming.set(false),
                        "Keep"
                    }
                }
            } else {
                button {
                    class: "btn btn-ghost btn-sm {Styles::club_leave_btn}",
                    onclick: move |_| confirming.set(true),
                    if club.accepted { "Leave" } else { "Cancel request" }
                    i { class: "ph ph-x {Styles::club_leave_icon}" }
                }
            }
            if let Some(err) = error.read().as_ref() {
                span { class: Styles::club_join_error, "{err}" }
            }
        }
    }
}

#[component]
fn ClubPublicCard(club: ClubItem, #[props(default)] description: Option<String>) -> Element {
    let display = club
        .display_name
        .as_deref()
        .unwrap_or(&club.handle)
        .to_string();
    let initial = initial_char(&display);
    let handle_at = format!("@{}@jogga.fit", club.handle);
    let mut copied = use_signal(|| false);
    let copy_handle = handle_at.clone();

    rsx! {
        ClubProfileCard {
            display,
            initial,
            handle_at,
            description,
            stats: rsx! {
                if club.exclusive {
                    span { class: "{Styles::club_badge} {Styles::club_badge_exclusive}", "Exclusive" }
                } else {
                    span { class: "{Styles::club_badge} {Styles::club_badge_open}", "Open" }
                }
                span { class: Styles::club_member_count,
                    i { class: "ph ph-users" }
                    " {club.member_count}"
                }
            },
            button {
                class: if *copied.read() { "btn btn-ghost" } else { "btn btn-primary" },
                onclick: move |_| {
                    let h = copy_handle.clone();
                    spawn(async move {
                        if copy_to_clipboard(&h).await.is_ok() {
                            copied.set(true);
                            sleep_ms(1_500).await;
                            copied.set(false);
                        }
                    });
                },
                if *copied.read() { "✓ Copied!" } else { "Join" }
            }
        }
    }
}

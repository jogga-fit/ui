use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    AcceptFollowRequestFn, FollowArgs, FollowPersonFn, FollowRequestArgs, KickFollowerArgs,
    KickFollowerFn, RejectFollowRequestFn, TokenApIdArgs, UnfollowActorFn,
    components::{
        avatar::{Avatar, AvatarSize},
        person_row::PersonRow,
        remote_follow_card::RemoteFollowCard,
    },
    sleep_ms,
    types::FollowingItem,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DirectoryItem {
    pub username: String,
    pub domain: String,
    pub ap_id: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FollowerItem {
    pub ap_id: String,
    pub username: String,
    pub domain: String,
    pub is_local: bool,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub accepted: bool,
    /// Original AP Follow activity id URL — None for pre-migration follows.
    pub follow_ap_id: Option<String>,
}

#[css_module("/src/pages/people/style.css")]
struct Styles;

#[component]
pub fn PeoplePageView(
    is_logged_in: bool,
    token: String,
    following_items: Vec<FollowingItem>,
    follower_items: Vec<FollowerItem>,
    pending_items: Vec<FollowerItem>,
    on_following_change: EventHandler<()>,
    on_follower_change: EventHandler<()>,
    follow_person_fn: FollowPersonFn,
    unfollow_actor_fn: UnfollowActorFn,
    kick_follower_fn: KickFollowerFn,
    accept_follow_request_fn: AcceptFollowRequestFn,
    reject_follow_request_fn: RejectFollowRequestFn,
    #[props(default)] directory_items: Vec<DirectoryItem>,
) -> Element {
    rsx! {
        div { class: "page-content",
            h1 {
                class: "settings-title", "People",
                if !is_logged_in && !directory_items.is_empty() { span { class: Styles::tab_count, "{directory_items.len()}" } }
            }

            if is_logged_in {
                FollowCard {
                    token: token.clone(),
                    on_success: move |_| on_following_change.call(()),
                    follow_person_fn,
                }
            }

            if is_logged_in {
                ConnectionsCard {
                    token,
                    following_items,
                    follower_items,
                    pending_items,
                    on_following_change,
                    on_follower_change,
                    unfollow_actor_fn,
                    kick_follower_fn,
                    accept_follow_request_fn,
                    reject_follow_request_fn,
                }
            }

            if !directory_items.is_empty() {
                div { class: "card",
                    for item in &directory_items {
                        PersonRow {
                            username: item.username.clone(),
                            domain: item.domain.clone(),
                            display_name: item.display_name.clone(),
                            avatar_url: item.avatar_url.clone(),
                            is_local: item.domain == "jogga.fit",
                            bio: item.bio.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ConnectionsCard(
    token: String,
    following_items: Vec<FollowingItem>,
    follower_items: Vec<FollowerItem>,
    pending_items: Vec<FollowerItem>,
    on_following_change: EventHandler<()>,
    on_follower_change: EventHandler<()>,
    unfollow_actor_fn: UnfollowActorFn,
    kick_follower_fn: KickFollowerFn,
    accept_follow_request_fn: AcceptFollowRequestFn,
    reject_follow_request_fn: RejectFollowRequestFn,
) -> Element {
    let has_pending = !pending_items.is_empty();
    let mut active_tab: Signal<&'static str> = use_signal(|| "following");

    let n_following = following_items.len();
    let n_followers = follower_items.len();
    let n_pending = pending_items.len();

    rsx! {
        div { class: format!("card {}", Styles::connections_card),
            div { class: "modal-header",
                div { class: "modal-tabs",
                    button {
                        class: if *active_tab.read() == "following" { "modal-tab modal-tab-active" } else { "modal-tab" },
                        onclick: move |_| active_tab.set("following"),
                        "Following"
                        span { class: Styles::tab_count, "{n_following}" }
                    }
                    button {
                        class: if *active_tab.read() == "followers" { "modal-tab modal-tab-active" } else { "modal-tab" },
                        onclick: move |_| active_tab.set("followers"),
                        "Followers"
                        span { class: Styles::tab_count, "{n_followers}" }
                    }
                    if has_pending {
                        button {
                            class: if *active_tab.read() == "requests" { "modal-tab modal-tab-active" } else { "modal-tab" },
                            onclick: move |_| active_tab.set("requests"),
                            "Requests"
                            span { class: format!("{} {}", Styles::tab_count, Styles::tab_count_accent), "{n_pending}" }
                        }
                    }
                }
            }
            div { class: Styles::connections_list,
                match active_tab() {
                    "following" => rsx! {
                        FollowingTab {
                            token: token.clone(),
                            items: following_items.clone(),
                            on_change: move |_| on_following_change.call(()),
                            unfollow_actor_fn,
                        }
                    },
                    "followers" => rsx! {
                        FollowersTab {
                            token: token.clone(),
                            items: follower_items.clone(),
                            on_change: move |_| on_follower_change.call(()),
                            kick_follower_fn,
                        }
                    },
                    _ => rsx! {
                        RequestsTab {
                            token: token.clone(),
                            items: pending_items.clone(),
                            on_change: move |_| on_follower_change.call(()),
                            accept_follow_request_fn,
                            reject_follow_request_fn,
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn FollowingTab(
    token: String,
    items: Vec<FollowingItem>,
    on_change: EventHandler<()>,
    unfollow_actor_fn: UnfollowActorFn,
) -> Element {
    let mut confirming = use_signal(|| Option::<String>::None);
    let mut busy = use_signal(|| false);
    let nav = use_navigator();

    if items.is_empty() {
        return rsx! {
            div { class: "connections-empty", "Not following anyone yet." }
        };
    }

    rsx! {
        {items.iter().map(|item| {
            let display = item.display_name.as_deref().unwrap_or(&item.username).to_string();
            let ap_id = item.ap_id.clone();
            let ap_id_confirm = ap_id.clone();
            let ap_id_unfollow = ap_id.clone();
            let tok = token.clone();
            let username = item.username.clone();
            let domain = item.domain.clone();
            let is_local = item.is_local;
            let handle = if is_local { format!("@{username}") } else { format!("@{username}@{domain}") };
            let username_nav = username.clone();
            let is_confirming = confirming.read().as_deref() == Some(ap_id.as_str());
            let accepted = item.accepted;
            let avatar = item.avatar_url.clone();
            rsx! {
                div { class: Styles::follow_list_item, key: "{ap_id}",
                    div {
                        class: if is_local { Styles::follow_list_identity.to_string() } else { format!("{} {}", Styles::follow_list_identity, Styles::follow_list_identity_remote) },
                        onclick: move |_| {
                            if is_local {
                                nav.push(format!("/@{username_nav}"));
                            }
                        },
                        Avatar { url: avatar, name: display.clone(), size: AvatarSize::Small }
                        div { class: Styles::connection_info,
                            span { class: Styles::connection_name, "{display}" }
                            span { class: Styles::connection_handle, "{handle}" }
                        }
                    }
                    if !accepted {
                        span { class: format!("{} {}", Styles::follow_badge, Styles::follow_badge_pending), "pending" }
                    }
                    if is_confirming {
                        div { class: "unfollow-confirm",
                            span { class: "unfollow-confirm-text", "Unfollow?" }
                            button {
                                class: "btn btn-sm btn-danger",
                                disabled: *busy.read(),
                                onclick: move |_| {
                                    let token = tok.clone();
                                    let ap_id = ap_id_unfollow.clone();
                                    busy.set(true);
                                    spawn(async move {
                                        let _ = unfollow_actor_fn.call(TokenApIdArgs { token, ap_id }).await;
                                        busy.set(false);
                                        confirming.set(None);
                                        on_change.call(());
                                    });
                                },
                                if *busy.read() { "…" } else { "Confirm" }
                            }
                            button {
                                class: "btn btn-sm btn-ghost",
                                onclick: move |_| confirming.set(None),
                                "Cancel"
                            }
                        }
                    } else {
                        button {
                            class: format!("btn btn-sm btn-ghost {}", Styles::unfollow_btn),
                            onclick: move |_| confirming.set(Some(ap_id_confirm.clone())),
                            "Unfollow"
                        }
                    }
                }
            }
        })}
    }
}

#[component]
fn FollowersTab(
    token: String,
    items: Vec<FollowerItem>,
    on_change: EventHandler<()>,
    kick_follower_fn: KickFollowerFn,
) -> Element {
    let nav = use_navigator();

    if items.is_empty() {
        return rsx! {
            div { class: "connections-empty", "No followers yet." }
        };
    }

    rsx! {
        {items.iter().map(|item| {
            let display = item.display_name.as_deref().unwrap_or(&item.username).to_string();
            let ap_id = item.ap_id.clone();
            let tok = token.clone();
            let username = item.username.clone();
            let domain = item.domain.clone();
            let is_local = item.is_local;
            let handle = if is_local { format!("@{username}") } else { format!("@{username}@{domain}") };
            let username_nav = username.clone();
            let avatar = item.avatar_url.clone();
            rsx! {
                div { class: Styles::follow_list_item, key: "{ap_id}",
                    div {
                        class: if is_local { Styles::follow_list_identity.to_string() } else { format!("{} {}", Styles::follow_list_identity, Styles::follow_list_identity_remote) },
                        onclick: move |_| {
                            if is_local {
                                nav.push(format!("/@{username_nav}"));
                            }
                        },
                        Avatar { url: avatar, name: display.clone(), size: AvatarSize::Small }
                        div { class: Styles::connection_info,
                            span { class: Styles::connection_name, "{display}" }
                            span { class: Styles::connection_handle, "{handle}" }
                        }
                    }
                    button {
                        class: format!("btn btn-sm btn-ghost {}", Styles::remove_follower_btn),
                        title: "Remove follower",
                        onclick: move |_| {
                            let token = tok.clone();
                            let follower_ap_id = ap_id.clone();
                            spawn(async move {
                                let _ = kick_follower_fn.call(KickFollowerArgs { token, follower_ap_id }).await;
                                on_change.call(());
                            });
                        },
                        "✕"
                    }
                }
            }
        })}
    }
}

#[component]
fn RequestsTab(
    token: String,
    items: Vec<FollowerItem>,
    on_change: EventHandler<()>,
    accept_follow_request_fn: AcceptFollowRequestFn,
    reject_follow_request_fn: RejectFollowRequestFn,
) -> Element {
    let nav = use_navigator();

    if items.is_empty() {
        return rsx! {
            div { class: "connections-empty", "No pending requests." }
        };
    }

    rsx! {
        {items.iter().map(|item| {
            let display = item.display_name.as_deref().unwrap_or(&item.username).to_string();
            let ap_id = item.ap_id.clone();
            let follow_ap_id = item.follow_ap_id.clone().unwrap_or_default();
            let tok_a = token.clone();
            let tok_r = token.clone();
            let ap_id_r = ap_id.clone();
            let follow_ap_id_r = follow_ap_id.clone();
            let username = item.username.clone();
            let domain = item.domain.clone();
            let is_local = item.is_local;
            let handle = if is_local { format!("@{username}") } else { format!("@{username}@{domain}") };
            let username_nav = username.clone();
            let avatar = item.avatar_url.clone();
            rsx! {
                div { class: Styles::follow_list_item, key: "pending-{ap_id}",
                    div {
                        class: if is_local { Styles::follow_list_identity.to_string() } else { format!("{} {}", Styles::follow_list_identity, Styles::follow_list_identity_remote) },
                        onclick: move |_| {
                            if is_local {
                                nav.push(format!("/@{username_nav}"));
                            }
                        },
                        Avatar { url: avatar, name: display.clone(), size: AvatarSize::Small }
                        div { class: Styles::connection_info,
                            span { class: Styles::connection_name, "{display}" }
                            span { class: Styles::connection_handle, "{handle}" }
                        }
                    }
                    div { class: Styles::pending_actions,
                        button {
                            class: "btn btn-sm btn-accept",
                            title: "Accept",
                            onclick: move |_| {
                                let t = tok_a.clone();
                                let aid = ap_id.clone();
                                let fid = follow_ap_id.clone();
                                spawn(async move {
                                    let _ = accept_follow_request_fn.call(FollowRequestArgs {
                                        token: t,
                                        ap_id: aid,
                                        follow_ap_id: fid,
                                    })
                                    .await;
                                    on_change.call(());
                                });
                            },
                            "✓"
                        }
                        button {
                            class: "btn btn-sm btn-reject",
                            title: "Reject",
                            onclick: move |_| {
                                let t = tok_r.clone();
                                let aid = ap_id_r.clone();
                                let fid = follow_ap_id_r.clone();
                                spawn(async move {
                                    let _ = reject_follow_request_fn.call(FollowRequestArgs {
                                        token: t,
                                        ap_id: aid,
                                        follow_ap_id: fid,
                                    })
                                    .await;
                                    on_change.call(());
                                });
                            },
                            "✗"
                        }
                    }
                }
            }
        })}
    }
}

#[component]
fn FollowCard(
    token: String,
    on_success: EventHandler<()>,
    follow_person_fn: FollowPersonFn,
) -> Element {
    let mut target = use_signal(String::new);
    let mut following_signal = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);
    let mut done = use_signal(|| false);

    let on_follow = {
        let token = token.clone();
        move |_: ()| {
            let token = token.clone();
            let handle_or_url = target.read().trim().to_string();
            if handle_or_url.is_empty() {
                return;
            }
            following_signal.set(true);
            error.set(None);
            done.set(false);
            spawn(async move {
                match follow_person_fn
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
                        on_success.call(());
                    }
                    Err(e) => error.set(Some(e)),
                }
                following_signal.set(false);
            });
        }
    };

    rsx! {
        RemoteFollowCard {
            icon_class: "ph ph-user-plus".to_string(),
            title: "Follow someone".to_string(),
            description: "Paste an ActivityPub handle or profile URL to send a follow request from this instance.".to_string(),
            input_label: "Person handle or profile URL".to_string(),
            placeholder: "@runner@example.social".to_string(),
            value: target.read().clone(),
            examples: vec!["@alex@jogga.fit".to_string(), "https://example.social/users/alex".to_string()],
            button_icon: "ph ph-paper-plane-tilt".to_string(),
            button_label: "Follow".to_string(),
            busy_label: "Sending".to_string(),
            busy: *following_signal.read(),
            disabled: target.read().trim().is_empty(),
            error: error.read().clone(),
            success_message: if *done.read() { Some("Follow request sent".to_string()) } else { None },
            on_input: move |value: String| {
                target.set(value);
                done.set(false);
                error.set(None);
            },
            on_submit: on_follow,
        }
    }
}

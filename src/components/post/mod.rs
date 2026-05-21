use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub mod feed_card;
pub use feed_card::{EditPostModal, FeedCard};

use crate::{
    CreateReplyArgs, CreateReplyFn, DeleteObjectFn, LikeFn, RouteFn, TokenApIdArgs, UpdatePostFn,
    components::{
        actor_link::ActorLink, avatar::Avatar, error_banner::ErrorBanner,
        exercise_stats_grid::ExerciseStatsGrid, like_button::LikeButton,
        media_carousel::MediaCollage, post_menu::PostMenu,
    },
    format::format_published,
    types::FeedItem,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ThreadItem {
    pub ap_id: String,
    pub author_username: String,
    pub author_avatar_url: Option<String>,
    pub content: Option<String>,
    pub published: String,
    pub like_count: i64,
    pub viewer_has_liked: bool,
    pub viewer_is_owner: bool,
}

#[css_module("/src/components/post/style.css")]
struct Styles;

#[component]
pub fn PostCardHeader(
    actor_is_local: bool,
    actor_ap_id: String,
    actor_username: String,
    #[props(default)] actor_display_name: Option<String>,
    actor_domain: String,
    actor_avatar_url: Option<String>,
    profile_href: String,
    #[props(default)] club_href: Option<String>,
    #[props(default)] via_club_display: Option<String>,
    #[props(default)] stop_propagation: bool,
    post_icon: &'static str,
    post_label: String,
    published: String,
    show_menu: bool,
    deleting: bool,
    #[props(default)] on_edit: Option<EventHandler<()>>,
    on_delete: EventHandler<()>,
    #[props(optional)] extra_class: Option<String>,
) -> Element {
    rsx! {
        div {
            class: "feed-card-header",
            class: if let Some(extra_class) = &extra_class { "{extra_class}" },
            div { class: Styles::feed_actor_col,
                ActorLink {
                    is_local: actor_is_local,
                    ap_id: actor_ap_id.clone(),
                    username: actor_username.clone(),
                    display_name: actor_display_name.clone(),
                    domain: actor_domain.clone(),
                    avatar_url: actor_avatar_url.clone(),
                    profile_href: profile_href.clone(),
                    club_href: club_href.clone(),
                    via_club_display: via_club_display.clone(),
                    stop_propagation,
                }
                span { class: Styles::post_type_badge,
                    i { class: "ph {post_icon}" }
                    span { "{post_label}" }
                }
            }
            span { class: Styles::post_time, "{published}" }
            if show_menu {
                PostMenu {
                    deleting,
                    on_edit,
                    on_delete,
                }
            }
        }
    }
}

#[component]
pub fn ExerciseThreadPage(
    thread: Option<Result<(FeedItem, Vec<ThreadItem>), String>>,
    token: Option<String>,
    mut edit_open: Signal<bool>,
    on_parent_deleted: EventHandler<()>,
    on_thread_refresh: EventHandler<()>,
    delete_fn: DeleteObjectFn,
    like_fn: LikeFn,
    unlike_fn: LikeFn,
    route_fn: RouteFn,
    update_fn: UpdatePostFn,
    create_reply_fn: CreateReplyFn,
) -> Element {
    rsx! {
        div { class: "page-content thread-page",
            div { class: "thread-back",
                Link { class: "back-link", to: "/home",
                    "← Feed"
                }
            }
            match thread {
                None => rsx! { div { class: "loading", "Loading…" } },
                Some(Err(e)) => rsx! { ErrorBanner { message: e } },
                Some(Ok((parent, replies))) => rsx! {
                    ExerciseCard {
                        item: parent.clone(),
                        token: token.clone(),
                        on_deleted: move |_| on_parent_deleted.call(()),
                        on_edit_open: move |_| edit_open.set(true),
                        delete_fn,
                        like_fn,
                        unlike_fn,
                        route_fn,
                    }

                    if *edit_open.read() {
                        if let Some(tok) = token.clone() {
                            EditPostModal {
                                item: parent.clone(),
                                token: tok,
                                update_fn,
                                on_saved: move |_| {
                                    edit_open.set(false);
                                    on_thread_refresh.call(());
                                },
                                on_cancel: move |_| edit_open.set(false),
                            }
                        }
                    }

                    div { class: "thread-divider", "data-testid": "thread-divider",
                        if replies.is_empty() {
                            "No comments yet"
                        } else if replies.len() == 1 {
                            "1 comment"
                        } else {
                            "{replies.len()} comments"
                        }
                    }

                    {replies.iter().map(|r| rsx! {
                        ReplyItem {
                            key: "{r.ap_id}",
                            item: r.clone(),
                            token: token.clone(),
                            on_deleted: move |_| on_thread_refresh.call(()),
                            delete_fn,
                            like_fn,
                            unlike_fn,
                        }
                    })}

                    if token.is_some() {
                        ReplyComposer {
                            in_reply_to: parent.object_ap_id.clone(),
                            token: token.clone().unwrap_or_default(),
                            on_posted: move |_| on_thread_refresh.call(()),
                            create_reply_fn,
                        }
                    }
                },
            }
        }
    }
}

#[component]
pub fn ExerciseCard(
    item: FeedItem,
    token: Option<String>,
    on_deleted: EventHandler<()>,
    on_edit_open: EventHandler<()>,
    delete_fn: DeleteObjectFn,
    like_fn: LikeFn,
    unlike_fn: LikeFn,
    route_fn: RouteFn,
) -> Element {
    let published = format_published(&item.published);

    let (post_icon, post_label) = item
        .stats
        .exercise_type
        .as_ref()
        .map_or(("ph-medal", "Exercise"), |t| t.icon_label());

    let pace_s_per_km = item.stats.avg_pace_s_per_km.or_else(|| {
        match (item.stats.duration_s, item.stats.distance_m) {
            (Some(dur), Some(dist)) if dist > 0.0 => Some(dur as f64 / (dist / 1000.0)),
            _ => None,
        }
    });

    let mut deleting = use_signal(|| false);
    let delete_ap_id = item.object_ap_id.clone();
    let delete_tok = token.clone();
    let is_owner = item.viewer_is_owner;
    let content_html = {
        let content = item.content.clone();
        use_memo(move || {
            content
                .as_deref()
                .filter(|s| !s.is_empty())
                .map(ammonia::clean)
        })
    };

    let do_delete = move |_| {
        let token = match delete_tok.clone() {
            Some(t) => t,
            None => return,
        };
        let ap_id = delete_ap_id.clone();
        deleting.set(true);
        spawn(async move {
            if delete_fn.call(TokenApIdArgs { token, ap_id }).await.is_ok() {
                on_deleted.call(());
            }
            deleting.set(false);
        });
    };

    rsx! {
        div { class: "card exercise-detail-card",
            if item.route_url.is_some() || !item.image_urls.is_empty() {
                MediaCollage {
                    route_url: item.route_url.clone(),
                    image_urls: item.image_urls.clone(),
                    token: token.clone(),
                    map_height: "300px".to_string(),
                    interactive: true,
                    route_fn,
                }
            }

            PostCardHeader {
                actor_is_local: item.actor_is_local,
                actor_ap_id: item.actor_ap_id.clone(),
                actor_username: item.actor_username.clone(),
                actor_display_name: item.actor_display_name.clone(),
                actor_domain: item.actor_domain.clone(),
                actor_avatar_url: item.actor_avatar_url.clone(),
                profile_href: format!("/@{}", item.actor_username),
                post_icon,
                post_label: post_label.to_string(),
                published,
                show_menu: is_owner && token.is_some(),
                deleting: *deleting.read(),
                on_edit: move |_| on_edit_open.call(()),
                on_delete: do_delete,
                extra_class: "edc-section",
            }

            div { class: format!("{} edc-section", Styles::exercise_detail_header),
                if let Some(ref title) = item.title {
                    h2 { class: "{Styles::exercise_detail_title}", "{title}" }
                }
                if let Some(ref dev) = item.stats.device {
                    span { class: "{Styles::exercise_device_badge}",
                        i { class: "ph ph-watch" }
                        "{dev}"
                    }
                }
            }

            ExerciseStatsGrid {
                distance_m: item.stats.distance_m,
                duration_s: item.stats.duration_s,
                pace_s_per_km,
                elevation_gain_m: item.stats.elevation_gain_m,
                avg_heart_rate_bpm: item.stats.avg_heart_rate_bpm,
                max_heart_rate_bpm: item.stats.max_heart_rate_bpm,
                avg_power_w: item.stats.avg_power_w,
                max_power_w: item.stats.max_power_w,
                normalized_power_w: item.stats.normalized_power_w,
                avg_cadence_rpm: item.stats.avg_cadence_rpm,
                extra_class: "stats-grid-detail edc-section",
            }

            if let Some(html) = content_html.read().as_deref() {
                div { class: "feed-content edc-section",
                    p { dangerous_inner_html: "{html}" }
                }
            }

            div { class: "feed-card-actions edc-section",
                LikeButton {
                    object_ap_id: item.object_ap_id.clone(),
                    token: token.clone(),
                    initial_liked: item.viewer_has_liked,
                    initial_count: item.like_count,
                    like_fn,
                    unlike_fn,
                }
            }
        }
    }
}

#[component]
pub fn ReplyItem(
    item: ThreadItem,
    token: Option<String>,
    on_deleted: EventHandler<()>,
    delete_fn: DeleteObjectFn,
    like_fn: LikeFn,
    unlike_fn: LikeFn,
) -> Element {
    let published = format_published(&item.published);

    let mut deleting = use_signal(|| false);
    let delete_ap_id = item.ap_id.clone();
    let delete_tok = token.clone();
    let is_owner = item.viewer_is_owner;
    let content_html = {
        let content = item.content.clone();
        use_memo(move || {
            content
                .as_deref()
                .filter(|s| !s.is_empty())
                .map(ammonia::clean)
        })
    };

    let do_delete = move |_| {
        let token = match delete_tok.clone() {
            Some(t) => t,
            None => return,
        };
        let ap_id = delete_ap_id.clone();
        deleting.set(true);
        spawn(async move {
            if delete_fn.call(TokenApIdArgs { token, ap_id }).await.is_ok() {
                on_deleted.call(());
            }
            deleting.set(false);
        });
    };

    rsx! {
        div { class: "card reply-item",
            div { class: "feed-card-header",
                div { class: "feed-avatar-wrap",
                    Avatar { url: item.author_avatar_url.clone(), name: item.author_username.clone() }
                    span { class: "feed-username", "@{item.author_username}" }
                }
                span { class: "post-time", "{published}" }
                if is_owner && token.is_some() {
                    PostMenu {
                        deleting: *deleting.read(),
                        on_delete: do_delete,
                    }
                }
            }
            if let Some(html) = content_html.read().as_deref() {
                div { class: "feed-content",
                    p { dangerous_inner_html: "{html}" }
                }
            }
            div { class: "feed-card-actions",
                LikeButton {
                    object_ap_id: item.ap_id.clone(),
                    token: token.clone(),
                    initial_liked: item.viewer_has_liked,
                    initial_count: item.like_count,
                    like_fn,
                    unlike_fn,
                }
            }
        }
    }
}

#[component]
pub fn ReplyComposer(
    in_reply_to: String,
    token: String,
    on_posted: EventHandler<()>,
    create_reply_fn: CreateReplyFn,
) -> Element {
    let mut content = use_signal(String::new);
    let mut submitting = use_signal(|| false);
    let mut error: Signal<Option<String>> = use_signal(|| None);

    let submit = {
        let in_reply_to = in_reply_to.clone();
        let tok = token.clone();
        move |_| {
            let text = content.read().trim().to_string();
            if text.is_empty() || *submitting.read() {
                return;
            }
            let in_reply_to = in_reply_to.clone();
            let token = tok.clone();
            submitting.set(true);
            error.set(None);
            spawn(async move {
                match create_reply_fn
                    .call(CreateReplyArgs {
                        token,
                        content: text,
                        in_reply_to,
                    })
                    .await
                {
                    Ok(()) => {
                        content.set(String::new());
                        on_posted.call(());
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                submitting.set(false);
            });
        }
    };

    rsx! {
        div { class: "{Styles::reply_composer}", "data-testid": "reply-composer",
            if let Some(err) = error.read().clone() {
                ErrorBanner { message: err }
            }
            textarea {
                class: "reply-textarea",
                placeholder: "Write a reply…",
                disabled: *submitting.read(),
                value: "{content}",
                oninput: move |e| content.set(e.value()),
            }
            button {
                class: "btn btn-primary",
                disabled: content.read().trim().is_empty() || *submitting.read(),
                onclick: submit,
                if *submitting.read() { "Posting…" } else { "Reply" }
            }
        }
    }
}

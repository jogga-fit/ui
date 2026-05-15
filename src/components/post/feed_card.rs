use dioxus::prelude::*;

#[css_module("/src/components/post/feed_card.css")]
struct Styles;

use super::PostCardHeader;
use crate::{
    DeleteObjectFn, LikeFn, RouteFn, TokenApIdArgs, UpdatePostArgs, UpdatePostFn,
    components::{
        error_banner::ErrorBanner,
        exercise_stats_grid::ExerciseStatsGrid,
        like_button::LikeButton,
        media_carousel::{CarouselOverlay, MediaCollage},
        stat_toggle_chips::{STAT_TOGGLES, StatToggleChips},
    },
    format::format_published,
    types::{FeedItem, ObjectType},
};

fn sanitize_path_segment(s: &str) -> String {
    s.replace('/', "%2F")
        .replace('?', "%3F")
        .replace('#', "%23")
}

#[component]
pub fn FeedCard(
    item: FeedItem,
    token: Option<String>,
    #[props(default)] on_deleted: Option<EventHandler<()>>,
    #[props(default)] on_edited: Option<EventHandler<()>>,
    #[props(default)] on_share_open: Option<EventHandler<()>>,
    delete_fn: DeleteObjectFn,
    like_fn: LikeFn,
    unlike_fn: LikeFn,
    route_fn: RouteFn,
    update_fn: UpdatePostFn,
) -> Element {
    if item.object_type != ObjectType::Exercise {
        return rsx! {};
    }

    let published = format_published(&item.published);

    let (post_icon, post_label) = {
        let (icon, label) = item
            .stats
            .exercise_type
            .as_ref()
            .map_or(("ph-medal", "Exercise"), |t| t.icon_label());
        (icon, label.to_string())
    };

    let mut deleting = use_signal(|| false);
    let mut edit_open = use_signal(|| false);
    let delete_ap_id = item.object_ap_id.clone();
    let delete_tok = token.clone();
    let is_owner = item.viewer_is_owner;

    let do_delete = move |_: ()| {
        let token = match delete_tok.clone() {
            Some(t) => t,
            None => return,
        };
        let ap_id = delete_ap_id.clone();
        deleting.set(true);
        spawn(async move {
            if (delete_fn.0)(TokenApIdArgs { token, ap_id }).await.is_ok() {
                if let Some(cb) = on_deleted {
                    cb.call(());
                }
            }
            deleting.set(false);
        });
    };

    let uuid = sanitize_path_segment(item.object_ap_id.rsplit('/').next().unwrap_or(""));
    let actor_handle = if item.actor_is_local {
        format!("@{}", sanitize_path_segment(&item.actor_username))
    } else {
        format!(
            "@{}@{}",
            sanitize_path_segment(&item.actor_username),
            sanitize_path_segment(&item.actor_domain),
        )
    };
    let detail_url = match item.object_type {
        ObjectType::Exercise => format!("/{actor_handle}/exercises/{uuid}"),
        _ => format!("/{actor_handle}/notes/{uuid}"),
    };
    let nav = use_navigator();

    let mut overlay_open: Signal<Option<usize>> = use_signal(|| None);

    rsx! {
        div {
            class: format!("card {} {}", Styles::feed_card, Styles::feed_card_clickable),
            "data-testid": "feed-card",
            onclick: {
                let detail_url = detail_url.clone();
                move |_| { nav.push(detail_url.clone()); }
            },
            PostCardHeader {
                actor_is_local: item.actor_is_local,
                actor_ap_id: item.actor_ap_id.clone(),
                actor_username: item.actor_username.clone(),
                actor_display_name: item.actor_display_name.clone(),
                actor_domain: item.actor_domain.clone(),
                actor_avatar_url: item.actor_avatar_url.clone(),
                profile_href: format!("/@{}", item.actor_username),
                club_href: item.via_club_handle.as_ref().map(|h| format!("/clubs/{h}")),
                via_club_display: item.via_club_display.clone(),
                stop_propagation: true,
                post_icon,
                post_label: post_label.clone(),
                published: published.clone(),
                show_menu: is_owner && token.is_some(),
                deleting: *deleting.read(),
                on_edit: move |_| edit_open.set(true),
                on_delete: do_delete,
            }

            if item.object_type == ObjectType::Exercise {
                if let Some(ref title) = item.title {
                    h3 { class: Styles::exercise_title, "{title}" }
                }
            }

            if item.object_type == ObjectType::Exercise {
                ExerciseStatsGrid {
                    distance_m: item.stats.distance_m,
                    duration_s: item.stats.duration_s,
                    pace_s_per_km: item.stats.avg_pace_s_per_km,
                    elevation_gain_m: item.stats.elevation_gain_m,
                    avg_heart_rate_bpm: item.stats.avg_heart_rate_bpm,
                    max_heart_rate_bpm: item.stats.max_heart_rate_bpm,
                    avg_power_w: item.stats.avg_power_w,
                    max_power_w: item.stats.max_power_w,
                    normalized_power_w: item.stats.normalized_power_w,
                    avg_cadence_rpm: item.stats.avg_cadence_rpm,
                }
            }

            if let Some(content) = &item.content {
                if !content.is_empty() {
                    div { class: "feed-content",
                        p { dangerous_inner_html: ammonia::clean(content) }
                    }
                }
            }

            if item.route_url.is_some() || !item.image_urls.is_empty() {
                div { onclick: move |e| e.stop_propagation(),
                    MediaCollage {
                        route_url: item.route_url.clone(),
                        image_urls: item.image_urls.clone(),
                        token: token.clone(),
                        on_open_overlay: move |idx| overlay_open.set(Some(idx)),
                        route_fn,
                    }
                }
            }

            if *edit_open.read() {
                if let Some(tok) = token.clone() {
                    EditPostModal {
                        item: item.clone(),
                        token: tok,
                        update_fn,
                        on_saved: move |_| {
                            edit_open.set(false);
                            if let Some(cb) = on_edited { cb.call(()); }
                        },
                        on_cancel: move |_| edit_open.set(false),
                    }
                }
            }

            div { class: "feed-card-actions",
                LikeButton {
                    object_ap_id: item.object_ap_id.clone(),
                    token: token.clone(),
                    initial_liked: item.viewer_has_liked,
                    initial_count: item.like_count,
                    like_fn,
                    unlike_fn,
                    stop_propagation: true,
                }
                if item.in_reply_to.is_none() {
                    Link {
                        class: Styles::reply_count_link.to_string(),
                        to: detail_url.clone(),
                        onclick: move |e: MouseEvent| e.stop_propagation(),
                        aria_label: if item.reply_count > 0 { format!("{} replies", item.reply_count) } else { "Reply".to_string() },
                        i { class: "ph ph-chat-circle {Styles::reply_icon}" }
                        if item.reply_count > 0 {
                            span { class: Styles::reply_count, "{item.reply_count}" }
                        }
                    }
                }
                if let Some(ref share_handler) = on_share_open {
                    button {
                        class: "boost-btn",
                        onclick: {
                            let share_handler = *share_handler;
                            move |e: MouseEvent| {
                                e.stop_propagation();
                                share_handler.call(());
                            }
                        },
                        title: "Share to club",
                        aria_label: "Share to club",
                        i { class: "ph ph-megaphone boost-icon" }
                    }
                }
            }
        }
        if let Some(idx) = *overlay_open.read() {
            CarouselOverlay {
                route_url: item.route_url.clone(),
                image_urls: item.image_urls.clone(),
                token: token.clone(),
                initial_index: idx,
                on_close: move |_| overlay_open.set(None),
                route_fn,
            }
        }
    }
}

#[component]
pub fn EditPostModal(
    item: FeedItem,
    token: String,
    update_fn: UpdatePostFn,
    on_saved: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    let is_exercise = item.object_type == ObjectType::Exercise;

    let mut title = use_signal(|| item.title.clone().unwrap_or_default());
    let mut content = use_signal(|| item.content.clone().unwrap_or_default());
    let hidden_stats: Signal<Vec<String>> = use_signal(|| item.hidden_stats.clone());
    let mut removed_urls: Signal<Vec<String>> = use_signal(Vec::new);
    let mut saving = use_signal(|| false);
    let mut error: Signal<Option<String>> = use_signal(|| None);

    let has_map = item.route_url.is_some() || item.hidden_stats.contains(&"map".to_string());

    let relevant_stats: Vec<(String, String)> = STAT_TOGGLES
        .iter()
        .copied()
        .filter(|(key, _)| {
            let k = key.to_string();
            match *key {
                "avg_heart_rate_bpm" => {
                    item.stats.avg_heart_rate_bpm.is_some() || item.hidden_stats.contains(&k)
                }
                "max_heart_rate_bpm" => {
                    item.stats.max_heart_rate_bpm.is_some() || item.hidden_stats.contains(&k)
                }
                "avg_power_w" => item.stats.avg_power_w.is_some() || item.hidden_stats.contains(&k),
                "max_power_w" => item.stats.max_power_w.is_some() || item.hidden_stats.contains(&k),
                "normalized_power_w" => {
                    item.stats.normalized_power_w.is_some() || item.hidden_stats.contains(&k)
                }
                "avg_cadence_rpm" => {
                    item.stats.avg_cadence_rpm.is_some() || item.hidden_stats.contains(&k)
                }
                _ => false,
            }
        })
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let show_toggles = is_exercise && (has_map || !relevant_stats.is_empty());

    let tok = token.clone();
    let object_ap_id = item.object_ap_id.clone();

    let do_save = move |_| {
        if *saving.read() {
            return;
        }
        let token = tok.clone();
        let object_ap_id = object_ap_id.clone();
        let c = content.read().trim().to_string();
        let title = if is_exercise {
            Some(title.read().trim().to_string())
        } else {
            None
        };
        let hidden_stats = if is_exercise {
            hidden_stats.read().clone()
        } else {
            vec![]
        };
        let removed_urls = removed_urls.read().clone();
        saving.set(true);
        error.set(None);
        spawn(async move {
            let content = if c.is_empty() { None } else { Some(c) };
            match (update_fn.0)(UpdatePostArgs {
                token,
                object_ap_id,
                content,
                title,
                hidden_stats,
                removed_urls,
            })
            .await
            {
                Ok(_) => on_saved.call(()),
                Err(e) => {
                    error.set(Some(e));
                    saving.set(false);
                }
            }
        });
    };

    rsx! {
        div {
            class: "modal-backdrop",
            onclick: move |_| on_cancel.call(()),
            div {
                class: format!("modal-card {}", Styles::edit_modal),
                onclick: move |e| e.stop_propagation(),

                div { class: format!("modal-header {}", Styles::edit_modal_header),
                    span { class: Styles::edit_modal_title,
                        if is_exercise { "Edit Activity" } else { "Edit Post" }
                    }
                    button { class: "modal-close", onclick: move |_| on_cancel.call(()), "×" }
                }

                div { class: Styles::edit_modal_body,
                    if let Some(err) = error.read().clone() {
                        ErrorBanner { message: err }
                    }

                    if is_exercise {
                        div { class: Styles::edit_field,
                            label { class: Styles::edit_label, "Title" }
                            input {
                                r#type: "text",
                                class: "activity-title-input",
                                value: "{title}",
                                oninput: move |e| title.set(e.value()),
                                disabled: *saving.read(),
                            }
                        }
                    }

                    div { class: Styles::edit_field,
                        label { class: Styles::edit_label,
                            if is_exercise { "Description" } else { "Content" }
                        }
                        textarea {
                            class: "reply-textarea",
                            rows: "4",
                            value: "{content}",
                            oninput: move |e| content.set(e.value()),
                            disabled: *saving.read(),
                        }
                    }

                    if show_toggles {
                        StatToggleChips {
                            has_map,
                            stats: relevant_stats,
                            hidden_stats,
                        }
                    }

                    if !item.image_urls.is_empty() {
                        div { class: Styles::edit_field,
                            label { class: Styles::edit_label, "Photos" }
                            div { class: Styles::edit_image_strip,
                                {item.image_urls.iter().map(|url| {
                                    let u = url.clone();
                                    let u2 = url.clone();
                                    let u3 = url.clone();
                                    rsx! {
                                        div {
                                            key: "{u3}",
                                            class: if removed_urls.read().contains(&u) { format!("{} {}", Styles::edit_thumb_wrap, Styles::edit_thumb_removed) } else { Styles::edit_thumb_wrap.to_string() },
                                            img { class: Styles::edit_thumb, src: "{u2}", alt: "" }
                                            button {
                                                class: "compose-thumb-remove",
                                                r#type: "button",
                                                title: if removed_urls.read().contains(&u) { "Restore" } else { "Remove" },
                                                onclick: move |_| {
                                                    let mut rv = removed_urls.write();
                                                    if rv.contains(&u) { rv.retain(|x| x != &u); } else { rv.push(u.clone()); }
                                                },
                                                if removed_urls.read().contains(&u3) { "↩" } else { "×" }
                                            }
                                        }
                                    }
                                })}
                            }
                        }
                    }
                }

                div { class: Styles::edit_modal_footer,
                    button {
                        class: "btn btn-ghost btn-sm",
                        onclick: move |_| on_cancel.call(()),
                        disabled: *saving.read(),
                        "Cancel"
                    }
                    button {
                        class: "btn btn-primary btn-sm",
                        onclick: do_save,
                        disabled: *saving.read(),
                        if *saving.read() { "Saving…" } else { "Save" }
                    }
                }
            }
        }
    }
}

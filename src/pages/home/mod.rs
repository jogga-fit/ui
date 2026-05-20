use dioxus::prelude::*;

use dioxus_leaflet::LatLng;

use crate::{
    ActivityVisibility, DeleteObjectFn, LikeFn, RouteFn, UpdatePostFn,
    components::{
        empty_state::EmptyState,
        error_banner::ErrorBanner,
        exercise_stats_grid::ExerciseStatsGrid,
        feed_gate::FeedGate,
        post::FeedCard,
        route_map::RouteMapFromCoords,
        stat_toggle_chips::{StatToggleChips, Styles as StatToggleChipStyles},
    },
    types::FeedItem,
};

#[css_module("/src/pages/home/style.css")]
struct Styles;

#[derive(Clone, PartialEq)]
pub struct PendingImagePreview {
    pub name: String,
    pub preview_url: String,
}

#[derive(Clone, PartialEq)]
pub struct ActivityPreview {
    pub route_coords: Vec<(f64, f64)>,
    pub distance_m: f64,
    pub duration_s: i64,
    pub elevation_gain_m: Option<f64>,
    pub avg_pace_s_per_km: Option<f64>,
    pub avg_heart_rate_bpm: Option<i32>,
    pub max_heart_rate_bpm: Option<i32>,
    pub avg_power_w: Option<f64>,
    pub max_power_w: Option<f64>,
    pub normalized_power_w: Option<f64>,
    pub avg_cadence_rpm: Option<f64>,
    pub device: Option<String>,
    pub present_stats: Vec<(String, String)>,
}

#[component]
pub fn HomePageView(
    is_logged_in: bool,
    composer: Element,
    feed: Option<Result<Vec<FeedItem>, String>>,
    token: Option<String>,
    on_feed_refresh: EventHandler<()>,
    delete_fn: DeleteObjectFn,
    like_fn: LikeFn,
    unlike_fn: LikeFn,
    route_fn: RouteFn,
    update_fn: UpdatePostFn,
    on_signin: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "page-content",
            if is_logged_in {
                {composer}
            }
            div { class: Styles::feed,
                match feed {
                    None => rsx! {
                        div { class: "loading-spinner", "Loading feed…" }
                    },
                    Some(Err(_)) => rsx! {
                        ErrorBanner { message: "Could not load feed. Please try again.".to_string() }
                    },
                    Some(Ok(items)) => {
                        if items.is_empty() {
                            rsx! {
                                EmptyState {
                                    icon: "ph ph-flag-checkered".to_string(),
                                    title: "Nothing here yet".to_string(),
                                    p { "Post your first activity above, or go to " Link { to: "/people", "People" } " to follow someone." }
                                }
                            }
                        } else if is_logged_in {
                            rsx! {
                                {items.iter().map(|item| rsx! {
                                    FeedCard {
                                        key: "{item.id}",
                                        item: item.clone(),
                                        token: token.clone(),
                                        on_deleted: move |_| on_feed_refresh.call(()),
                                        on_edited: move |_| on_feed_refresh.call(()),
                                        delete_fn,
                                        like_fn,
                                        unlike_fn,
                                        route_fn,
                                        update_fn,
                                    }
                                })}
                            }
                        } else {
                            rsx! {
                                {items.iter().take(3).map(|item| rsx! {
                                    FeedCard {
                                        key: "{item.id}",
                                        item: item.clone(),
                                        token: None,
                                        on_deleted: move |_| {},
                                        on_edited: move |_| {},
                                        delete_fn,
                                        like_fn,
                                        unlike_fn,
                                        route_fn,
                                        update_fn,
                                    }
                                })}
                                FeedGate {
                                    icon: "ph ph-person-simple-run",
                                    title: "Track your activities",
                                    description: "Sign in to see your full feed, follow friends, and post workouts.",
                                    content: rsx! {
                                        {items.iter().skip(3).take(1).map(|item| rsx! {
                                            FeedCard {
                                                key: "gate-{item.id}",
                                                item: item.clone(),
                                                token: None,
                                                on_deleted: move |_| {},
                                                on_edited: move |_| {},
                                                delete_fn,
                                                like_fn,
                                                unlike_fn,
                                                route_fn,
                                                update_fn,
                                            }
                                        })}
                                    },
                                   {rsx! {
                                        button { class: "btn btn-primary", onclick: move |_| on_signin.call(()), "Sign in" }
                                        a { class: "btn btn-ghost", href: "/register", "Create account" }
                                   }}
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn ActivityComposer(
    compose_error: Option<String>,
    activity_types: Vec<(String, String)>,
    activity_file_name: Option<String>,
    mut activity_type: Signal<String>,
    mut activity_title: Signal<String>,
    mut activity_desc: Signal<String>,
    mut activity_visibility: Signal<ActivityVisibility>,
    hidden_stats: Signal<Vec<String>>,
    preview: Option<ActivityPreview>,
    pending_images: Vec<PendingImagePreview>,
    images_loading: bool,
    posting: bool,
    file_ready: bool,
    on_images_changed: EventHandler<Event<FormData>>,
    on_activity_file_changed: EventHandler<Event<FormData>>,
    on_clear_activity_file: EventHandler<()>,
    on_remove_image: EventHandler<usize>,
    on_upload_activity: EventHandler<Event<MouseData>>,
) -> Element {
    let img_count = pending_images.len();
    let has_route = preview
        .as_ref()
        .is_some_and(|preview| !preview.route_coords.is_empty());
    let map_hidden = hidden_stats.read().contains(&"map".to_string());
    let show_toggles = preview
        .as_ref()
        .is_some_and(|preview| has_route || !preview.present_stats.is_empty());

    rsx! {
        div { class: format!("card {}", Styles::compose_card),
            input {
                r#type: "file",
                id: "post-image-input",
                accept: "image/*",
                multiple: true,
                style: "display:none",
                onchange: on_images_changed,
            }

            if let Some(err) = compose_error {
                ErrorBanner { message: err }
            }

            div { class: Styles::compose_body,
                label {
                    class: Styles::file_drop_zone,
                    "data-testid": "file-drop-zone",
                    r#for: "activity-file",
                    if let Some(name) = activity_file_name.as_ref() {
                        div { class: Styles::file_selected,
                            i { class: format!("ph ph-file {}", Styles::file_icon) }
                            span { class: Styles::file_name, "{name}" }
                            button {
                                class: Styles::file_remove,
                                onclick: move |e| {
                                    e.prevent_default();
                                    on_clear_activity_file.call(());
                                },
                                "×"
                            }
                        }
                    } else {
                        div { class: Styles::file_prompt,
                            i { class: format!("ph ph-folder-open {}", Styles::file_icon_lg) }
                            span { class: Styles::file_prompt_text, "data-testid": "file-prompt-text", "Drop your GPX or FIT file here, or click to browse" }
                            span { class: Styles::file_hint, ".gpx · .fit — from Garmin, Wahoo, Strava, Komoot…" }
                        }
                    }
                    input {
                        r#type: "file",
                        id: "activity-file",
                        accept: ".gpx,.fit,application/gpx+xml,application/octet-stream",
                        style: "display:none",
                        onchange: on_activity_file_changed,
                    }
                }

                if activity_file_name.is_some() {
                    div { class: Styles::activity_type_picker,
                        label { r#for: "activity-type-select", "Activity type" }
                        select {
                            id: "activity-type-select",
                            class: Styles::activity_type_select,
                            value: "{activity_type.read()}",
                            onchange: move |e| activity_type.set(e.value()),
                            for (val, label) in activity_types.iter() {
                                option {
                                    key: "{val}",
                                    value: "{val}",
                                    selected: *activity_type.read() == *val,
                                    "{label}"
                                }
                            }
                        }
                    }

                    if let Some(ref preview) = preview {
                        if has_route && !map_hidden {
                            {
                                let coords: Vec<LatLng> = preview
                                    .route_coords
                                    .iter()
                                    .map(|(lat, lon)| LatLng::new(*lat, *lon))
                                    .collect();
                                rsx! {
                                    RouteMapFromCoords {
                                        coords,
                                        map_height: "180px".to_string(),
                                    }
                                }
                            }
                        }
                        ExerciseStatsGrid {
                            distance_m: Some(preview.distance_m),
                            duration_s: Some(preview.duration_s),
                            pace_s_per_km: preview.avg_pace_s_per_km,
                            elevation_gain_m: preview.elevation_gain_m,
                            avg_heart_rate_bpm: preview.avg_heart_rate_bpm,
                            max_heart_rate_bpm: preview.max_heart_rate_bpm,
                            avg_power_w: preview.avg_power_w,
                            max_power_w: preview.max_power_w,
                            normalized_power_w: preview.normalized_power_w,
                            avg_cadence_rpm: preview.avg_cadence_rpm,
                        }
                        if let Some(ref d) = preview.device {
                            p { class: Styles::activity_device,
                                i { class: "ph ph-device-mobile-camera" }
                                " {d}"
                            }
                        }
                        if show_toggles {
                            StatToggleChips {
                                has_map: has_route,
                                stats: preview.present_stats.clone(),
                                hidden_stats,
                            }
                        }
                    }

                    div { class: Styles::note_compose_wrap,
                        input {
                            r#type: "text",
                            class: "activity-title-input",
                            placeholder: "Activity name — auto-generated if left blank",
                            value: "{activity_title}",
                            oninput: move |e| activity_title.set(e.value()),
                        }
                        textarea {
                            class: format!("{} {} {}", Styles::compose_input, Styles::note_compose_input, Styles::activity_desc_input),
                            placeholder: "How did it feel? Any notes about the route…",
                            rows: "3",
                            value: "{activity_desc}",
                            oninput: move |e| activity_desc.set(e.value()),
                        }
                        if img_count > 0 || images_loading {
                            NoteImageStrip {
                                images: pending_images.clone(),
                                loading: images_loading,
                                on_remove: on_remove_image,
                            }
                        }
                        div { class: Styles::note_compose_footer,
                            div { class: Styles::note_compose_footer_left,
                                if img_count < 8 && !images_loading {
                                    label {
                                        class: Styles::note_attach_btn,
                                        r#for: "post-image-input",
                                        title: "Add photos",
                                        i { class: "ph ph-paperclip" }
                                    }
                                }
                                div { class: StatToggleChipStyles::type_chip_row,
                                    for visibility in [ActivityVisibility::Public, ActivityVisibility::Followers, ActivityVisibility::Private] {
                                        button {
                                            key: "{visibility}",
                                            r#type: "button",
                                            class: if *activity_visibility.peek() == visibility {
                                                "{StatToggleChipStyles::type_chip} {StatToggleChipStyles::type_chip_sm} {StatToggleChipStyles::type_chip_active}"
                                            } else {
                                                "{StatToggleChipStyles::type_chip} {StatToggleChipStyles::type_chip_sm}"
                                            },
                                            onclick: move |_| activity_visibility.set(visibility),
                                            "{visibility}"
                                        }
                                    }
                                }
                            }
                            div { class: Styles::note_compose_footer_right,
                                button {
                                    class: "btn btn-primary btn-sm",
                                    disabled: posting || !file_ready,
                                    onclick: on_upload_activity,
                                    if posting { "Uploading…" } else { "Post" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NoteImageStrip(
    images: Vec<PendingImagePreview>,
    loading: bool,
    on_remove: EventHandler<usize>,
) -> Element {
    rsx! {
        div { class: Styles::note_image_strip,
            {images.iter().enumerate().map(|(i, img)| rsx! {
                div { key: "{i}", class: format!("{} {}", Styles::compose_thumb_wrap, Styles::note_thumb),
                    img {
                        class: Styles::compose_thumb,
                        src: "{img.preview_url}",
                        alt: "{img.name}",
                    }
                    button {
                        class: "compose-thumb-remove",
                        r#type: "button",
                        title: "Remove",
                        onclick: move |_| on_remove.call(i),
                        "×"
                    }
                }
            })}
            if loading {
                div { class: format!("{} {} {}", Styles::compose_thumb_wrap, Styles::note_thumb, Styles::compose_thumb_loading),
                    div { class: Styles::loading_spinner_sm }
                }
            }
        }
    }
}

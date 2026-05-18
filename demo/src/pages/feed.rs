#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::{
    DeleteObjectFn, LikeFn, RouteFn, UpdatePostFn,
    pages::{ActivityComposer, ActivityPreview, HomePageView},
    types::{AuthSignal, AuthUser, sleep_ms},
};

use crate::mock::{
    mock_delete, mock_like, mock_route, mock_update_post, ride_item,
    ride_with_photos_and_route_item, run_item, run_with_photos_item, run_with_route_item,
};

#[component]
pub fn FeedPage() -> Element {
    let mut auth = use_context::<AuthSignal>();
    let is_logged_in = auth.read().is_some();
    let token = auth.read().as_ref().map(|u| u.token.clone());

    let feed = vec![
        run_item(),
        ride_item(),
        run_with_route_item(),
        run_with_photos_item(),
        ride_with_photos_and_route_item(),
    ];

    let on_signin = move |_| {
        spawn(async move {
            let _ = document::eval(
                "document.cookie='jogga_auth=alex:demo-token; path=/; max-age=2592000; SameSite=Lax';"
            ).await;
        });
        auth.set(Some(AuthUser {
            token: "demo-token".to_string(),
            username: "alex".to_string(),
            ap_id: "https://jogga.fit/users/alex".to_string(),
        }));
    };

    let composer = if is_logged_in {
        rsx! { Composer {} }
    } else {
        rsx! {}
    };

    rsx! {
        HomePageView {
            is_logged_in,
            composer,
            feed: Some(Ok(feed)),
            token,
            on_feed_refresh: move |_| {},
            delete_fn: DeleteObjectFn(mock_delete),
            like_fn: LikeFn(mock_like),
            unlike_fn: LikeFn(mock_like),
            route_fn: RouteFn(mock_route),
            update_fn: UpdatePostFn(mock_update_post),
            on_signin,
        }
    }
}

#[component]
fn Composer() -> Element {
    let activity_type = use_signal(|| "run".to_string());
    let activity_title = use_signal(String::new);
    let activity_desc = use_signal(String::new);
    let activity_visibility = use_signal(|| "public".to_string());
    let hidden_stats = use_signal(Vec::<String>::new);
    let mut posting = use_signal(|| false);
    let mut file_name: Signal<Option<String>> = use_signal(|| None);

    let file_selected = file_name.read().is_some();
    let preview = file_selected.then(|| ActivityPreview {
        route_coords: vec![],
        distance_m: 10142.0,
        duration_s: 3720,
        elevation_gain_m: Some(112.0),
        avg_pace_s_per_km: Some(367.0),
        avg_heart_rate_bpm: Some(162),
        max_heart_rate_bpm: Some(181),
        avg_power_w: None,
        max_power_w: None,
        normalized_power_w: None,
        avg_cadence_rpm: Some(174.0),
        device: Some("Garmin Forerunner 955".to_string()),
        present_stats: vec![
            ("distance".to_string(), "km".to_string()),
            ("duration".to_string(), "time".to_string()),
            ("pace".to_string(), "min/km".to_string()),
            ("elevation".to_string(), "m".to_string()),
            ("heart_rate".to_string(), "bpm".to_string()),
        ],
    });

    rsx! {
        ActivityComposer {
            compose_error: None,
            activity_types: vec![
                ("run".to_string(), "Run".to_string()),
                ("ride".to_string(), "Ride".to_string()),
                ("swim".to_string(), "Swim".to_string()),
                ("hike".to_string(), "Hike".to_string()),
            ],
            activity_file_name: file_name.read().clone(),
            activity_type,
            activity_title,
            activity_desc,
            activity_visibility,
            hidden_stats,
            preview,
            pending_images: vec![],
            images_loading: false,
            posting: *posting.read(),
            file_ready: file_selected,
            on_images_changed: move |_| {},
            on_activity_file_changed: move |e: Event<FormData>| {
                // value() on a file input is "C:\fakepath\name" or "name"
                let raw = e.value();
                let name = raw
                    .split(['/', '\\'])
                    .last()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string());
                file_name.set(name);
            },
            on_clear_activity_file: move |_| {
                file_name.set(None);
            },
            on_remove_image: move |_| {},
            on_upload_activity: move |_| {
                posting.set(true);
                spawn(async move {
                    sleep_ms(1500).await;
                    posting.set(false);
                    file_name.set(None);
                });
            },
        }
    }
}

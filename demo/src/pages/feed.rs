#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::{
    DeleteObjectFn, LikeFn, RouteFn, UpdatePostFn,
    pages::HomePageView,
    types::{AuthSignal, AuthUser},
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

    rsx! {
        HomePageView {
            is_logged_in,
            composer: rsx! {},
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

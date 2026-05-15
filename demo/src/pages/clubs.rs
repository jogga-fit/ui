#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::{
    DeleteObjectFn, FollowActorFn, FutureResult, GetClubFeedFn, LikeFn, RouteFn, UnfollowActorFn,
    UpdatePostFn,
    pages::{ClubDetailPageView, ClubsPageView},
    types::{AuthSignal, FeedItem, sleep_ms},
};

use crate::mock::{
    mock_clubs, mock_delete, mock_follow_actor, mock_like, mock_route, mock_unfollow_actor,
    mock_update_post, ride_item,
};

#[component]
pub fn ClubsDemoPage() -> Element {
    let auth = use_context::<AuthSignal>();
    let is_logged_in = auth.read().is_some();
    let token = auth
        .read()
        .as_ref()
        .map(|u| u.token.clone())
        .unwrap_or_default();

    rsx! {
        ClubsPageView {
            token,
            clubs: Some(Ok(mock_clubs())),
            on_clubs_refresh: move |_| {},
            follow_actor_fn: FollowActorFn(mock_follow_actor),
            unfollow_actor_fn: UnfollowActorFn(mock_unfollow_actor),
            is_logged_in,
        }
    }
}

pub fn mock_get_club_feed(_handle: String, _token: Option<String>) -> FutureResult<Vec<FeedItem>> {
    Box::pin(async move {
        sleep_ms(400).await;
        Ok(vec![ride_item()])
    })
}

#[component]
pub fn ClubDetailPage(handle: String) -> Element {
    let auth = use_context::<AuthSignal>();
    let token = auth
        .read()
        .as_ref()
        .map(|u| u.token.clone())
        .unwrap_or_default();
    let nav = use_navigator();

    let club_handle = handle.clone();
    let club = mock_clubs().into_iter().find(|c| {
        let full_handle = format!("{}@{}", c.username, c.domain);
        c.username == club_handle || full_handle == club_handle
    });

    rsx! {
        ClubDetailPageView {
            handle: handle.clone(),
            club: Some(Ok(club)),
            token,
            on_back: move |_| { nav.push("/clubs"); },
            on_left: move |_| { nav.push("/clubs"); },
            get_club_feed_fn: GetClubFeedFn(mock_get_club_feed),
            unfollow_actor_fn: UnfollowActorFn(mock_unfollow_actor),
            delete_fn: DeleteObjectFn(mock_delete),
            like_fn: LikeFn(mock_like),
            unlike_fn: LikeFn(mock_like),
            route_fn: RouteFn(mock_route),
            update_fn: UpdatePostFn(mock_update_post),
        }
    }
}

#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::{
    FutureResult, GetClubFeedArgs,
    pages::{ClubDetailPageView, ClubsPageView},
    sleep_ms,
    types::{AuthSignal, FeedItem},
};

use crate::mock::{
    mock_clubs, mock_delete, mock_follow_actor, mock_like, mock_route, mock_server_clubs,
    mock_unfollow_actor, mock_update_post, ride_item,
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
            follow_actor_fn: Callback::new(mock_follow_actor),
            unfollow_actor_fn: Callback::new(mock_unfollow_actor),
            is_logged_in,
            server_clubs: mock_server_clubs(),
        }
    }
}

pub fn mock_get_club_feed(_args: GetClubFeedArgs) -> FutureResult<Vec<FeedItem>> {
    Box::pin(async move {
        sleep_ms(400).await;
        Ok(vec![ride_item()])
    })
}

#[component]
pub fn ClubDetailPage(handle: String) -> Element {
    let auth = use_context::<AuthSignal>();
    let is_logged_in = auth.read().is_some();
    let token = auth
        .read()
        .as_ref()
        .map(|u| u.token.clone())
        .unwrap_or_default();
    let nav = use_navigator();

    let club_handle = handle.clone();
    let club = if is_logged_in {
        mock_clubs().into_iter().find(|c| {
            let full_handle = format!("{}@{}", c.username, c.domain);
            c.username == club_handle || full_handle == club_handle
        })
    } else {
        None
    };

    let server_club = mock_server_clubs()
        .into_iter()
        .find(|c| c.handle == club_handle);
    let description = server_club.as_ref().and_then(|c| c.description.clone());

    rsx! {
        ClubDetailPageView {
            handle: handle.clone(),
            club: Some(Ok(club)),
            token,
            description,
            server_club,
            on_back: move |_| { nav.push("/clubs"); },
            on_left: move |_| { nav.push("/clubs"); },
            get_club_feed_fn: Callback::new(mock_get_club_feed),
            unfollow_actor_fn: Callback::new(mock_unfollow_actor),
            delete_fn: Callback::new(mock_delete),
            like_fn: Callback::new(mock_like),
            unlike_fn: Callback::new(mock_like),
            route_fn: Callback::new(mock_route),
            update_fn: Callback::new(mock_update_post),
        }
    }
}

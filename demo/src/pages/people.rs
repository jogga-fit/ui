#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::{pages::PeoplePageView, types::AuthSignal};

use crate::mock::{
    mock_accept_follow_request, mock_directory_items, mock_follow_person, mock_follower_items,
    mock_following_items, mock_kick_follower, mock_reject_follow_request, mock_unfollow_actor,
    pending_follow_requests,
};

#[component]
pub fn PeopleDemoPage() -> Element {
    let auth = use_context::<AuthSignal>();
    let is_logged_in = auth.read().is_some();
    let token = auth
        .read()
        .as_ref()
        .map(|u| u.token.clone())
        .unwrap_or_default();

    rsx! {
        PeoplePageView {
            is_logged_in,
            token,
            following_items: if is_logged_in { mock_following_items() } else { vec![] },
            follower_items: if is_logged_in { mock_follower_items() } else { vec![] },
            pending_items: if is_logged_in { pending_follow_requests() } else { vec![] },
            directory_items: mock_directory_items(),
            on_following_change: move |_| {},
            on_follower_change: move |_| {},
            follow_person_fn: Callback::new(mock_follow_person),
            unfollow_actor_fn: Callback::new(mock_unfollow_actor),
            kick_follower_fn: Callback::new(mock_kick_follower),
            accept_follow_request_fn: Callback::new(mock_accept_follow_request),
            reject_follow_request_fn: Callback::new(mock_reject_follow_request),
        }
    }
}

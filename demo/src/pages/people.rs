#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::{pages::PeoplePageView, types::AuthSignal};

use crate::mock::{
    mock_directory_items, mock_follower_items, mock_following_items, pending_follow_requests,
    people_fns,
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

    let (
        follow_person_fn,
        unfollow_actor_fn,
        kick_follower_fn,
        accept_follow_request_fn,
        reject_follow_request_fn,
    ) = people_fns();

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
            follow_person_fn,
            unfollow_actor_fn,
            kick_follower_fn,
            accept_follow_request_fn,
            reject_follow_request_fn,
        }
    }
}

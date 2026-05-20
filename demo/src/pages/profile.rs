#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::pages::ProfilePageView;

use crate::mock::{
    mock_check_following, mock_delete, mock_follow_person, mock_get_actor_connections,
    mock_get_actor_info, mock_get_actor_posts, mock_like, mock_route, mock_unfollow_actor,
    mock_update_post, mock_update_profile, mock_upload_avatar,
};

#[component]
pub fn ProfilePage() -> Element {
    rsx! {
        ProfilePageView {
            username: "alex".to_string(),
            get_actor_info_fn: Callback::new(mock_get_actor_info),
            get_actor_posts_fn: Callback::new(mock_get_actor_posts),
            get_actor_connections_fn: Callback::new(mock_get_actor_connections),
            check_following_fn: Callback::new(mock_check_following),
            update_profile_fn: Callback::new(mock_update_profile),
            upload_avatar_fn: Callback::new(mock_upload_avatar),
            follow_person_fn: Callback::new(mock_follow_person),
            unfollow_actor_fn: Callback::new(mock_unfollow_actor)   ,
            delete_fn: Callback::new(mock_delete),
            like_fn: Callback::new(mock_like),
            unlike_fn: Callback::new(mock_like),
            route_fn: Callback::new(mock_route),
            update_fn: Callback::new(mock_update_post),
        }
    }
}

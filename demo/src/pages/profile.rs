#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::{DeleteObjectFn, LikeFn, RouteFn, UpdatePostFn, pages::ProfilePageView};

use crate::mock::{mock_delete, mock_like, mock_route, mock_update_post, profile_fns};

#[component]
pub fn ProfilePage() -> Element {
    let (
        get_actor_info_fn,
        get_actor_posts_fn,
        get_actor_connections_fn,
        check_following_fn,
        update_profile_fn,
        upload_avatar_fn,
        follow_person_fn,
        unfollow_actor_fn,
    ) = profile_fns();

    rsx! {
        ProfilePageView {
            username: "alex".to_string(),
            get_actor_info_fn,
            get_actor_posts_fn,
            get_actor_connections_fn,
            check_following_fn,
            update_profile_fn,
            upload_avatar_fn,
            follow_person_fn,
            unfollow_actor_fn,
            delete_fn: DeleteObjectFn(mock_delete),
            like_fn: LikeFn(mock_like),
            unlike_fn: LikeFn(mock_like),
            route_fn: RouteFn(mock_route),
            update_fn: UpdatePostFn(mock_update_post),
        }
    }
}

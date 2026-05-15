#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::{
    CreateReplyFn, DeleteObjectFn, LikeFn, RouteFn, UpdatePostFn,
    components::post::ExerciseThreadPage,
};

use crate::mock::{
    mock_create_reply, mock_delete, mock_like, mock_route, mock_update_post, reply_item,
    reply_item_own, run_item,
};

#[component]
pub fn PostPage() -> Element {
    let edit_open = use_signal(|| false);

    rsx! {
        ExerciseThreadPage {
            thread: Some(Ok((run_item(), vec![reply_item(), reply_item_own()]))),
            token: Some("demo-token".to_string()),
            edit_open,
            on_parent_deleted: move |_| {},
            on_thread_refresh: move |_| {},
            delete_fn: DeleteObjectFn(mock_delete),
            like_fn: LikeFn(mock_like),
            unlike_fn: LikeFn(mock_like),
            route_fn: RouteFn(mock_route),
            update_fn: UpdatePostFn(mock_update_post),
            create_reply_fn: CreateReplyFn(mock_create_reply),
        }
    }
}

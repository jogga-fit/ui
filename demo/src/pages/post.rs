#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::components::post::ExerciseThreadPage;

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
            delete_fn: Callback::new(mock_delete),
            like_fn: Callback::new(mock_like),
            unlike_fn: Callback::new(mock_like),
            route_fn: Callback::new(mock_route),
            update_fn: Callback::new(mock_update_post),
            create_reply_fn: Callback::new(mock_create_reply),
        }
    }
}

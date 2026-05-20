use dioxus::prelude::*;

use crate::{LikeFn, TokenApIdArgs};

#[css_module("/src/components/like_button/style.css")]
struct Styles;

#[component]
pub fn LikeButton(
    object_ap_id: String,
    token: ReadSignal<Option<String>>,
    initial_liked: bool,
    initial_count: i64,
    like_fn: LikeFn,
    unlike_fn: LikeFn,
    #[props(default)] stop_propagation: bool,
) -> Element {
    let mut liked = use_signal(|| initial_liked);
    let mut like_count = use_signal(|| initial_count);
    let mut liking = use_signal(|| false);

    let stop_prop = stop_propagation;
    let toggle_like = move |e: MouseEvent| {
        if stop_prop {
            e.stop_propagation();
        }
        if *liking.peek() {
            return;
        }
        let Some(token) = token.peek().clone() else {
            return;
        };
        let ap_id = object_ap_id.clone();
        let currently_liked = *liked.peek();
        liked.set(!currently_liked);
        let new_count = *like_count.peek() + if currently_liked { -1 } else { 1 };
        like_count.set(new_count);
        liking.set(true);
        spawn(async move {
            let result = if currently_liked {
                unlike_fn.call(TokenApIdArgs { token, ap_id }).await
            } else {
                like_fn.call(TokenApIdArgs { token, ap_id }).await
            };
            if result.is_err() {
                liked.set(currently_liked);
                let rolled_back = *like_count.peek() + if currently_liked { 1 } else { -1 };
                like_count.set(rolled_back);
            }
            liking.set(false);
        });
    };

    rsx! {
        button {
            class: "{Styles::like_btn}",
            class: if *liked.read() { "{Styles::like_btn_active}" },
            disabled: token.read().is_none() || *liking.read(),
            onclick: toggle_like,
            title: if *liked.read() { "Unlike" } else { "Like" },
            aria_label: if *liked.read() { "Unlike" } else { "Like" },
            if *liked.read() {
                i { class: format!("ph-fill ph-heart {}", Styles::like_heart) }
            } else {
                i { class: format!("ph ph-heart {}", Styles::like_heart) }
            }
            if *like_count.read() > 0 {
                span { class: Styles::like_count, "{like_count}" }
            }
        }
    }
}

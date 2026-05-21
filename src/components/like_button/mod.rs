use std::ops::{AddAssign, SubAssign};

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
        let (change, action) = if currently_liked {
            (-1, unlike_fn)
        } else {
            (1, like_fn)
        };
        like_count.add_assign(change);
        liking.toggle();
        let args = TokenApIdArgs { token, ap_id };
        spawn(async move {
            if action.call(args).await.is_err() {
                liked.set(currently_liked);
                like_count.sub_assign(change);
            }
            liking.toggle();
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
            i {
                class: "{Styles::like_heart} ph-heart",
                class: if *liked.read() {
                    "ph-fill"
                } else {
                    "ph"
                },
            }
            if *like_count.read() > 0 {
                span { class: Styles::like_count, "{like_count}" }
            }
        }
    }
}

use dioxus::prelude::*;

use crate::{
    CheckFollowingFn, FollowArgs, FollowPersonFn, TokenApIdArgs, UnfollowActorFn,
    browser::copy_to_clipboard,
    components::error_banner::ErrorBanner,
    sleep_ms,
};

#[css_module("/src/components/follow_button/style.css")]
struct Styles;

#[component]
pub fn FollowButton(
    #[props(default)] is_own: bool,
    is_logged_in: bool,
    token: String,
    ap_id: String,
    #[props(default)] handle: String,
    #[props(default)] check_following_fn: Option<CheckFollowingFn>,
    follow_person_fn: FollowPersonFn,
    unfollow_actor_fn: UnfollowActorFn,
    /// Seed follow status when `check_following_fn` is absent.
    /// None = not following, Some(false) = pending, Some(true) = accepted.
    #[props(default)] initial_follow_status: Option<bool>,
    #[props(default)] on_followed: Option<EventHandler<()>>,
) -> Element {
    let has_check_fn = check_following_fn.is_some();

    // When check_following_fn is absent, seed immediately from initial_follow_status.
    // Outer None = still loading; Some(inner) = known state.
    let initial = if has_check_fn {
        None
    } else {
        Some(initial_follow_status)
    };

    let mut follow_status = use_signal(|| initial);
    let mut in_flight = use_signal(|| false);
    let mut follow_error = use_signal(|| Option::<String>::None);
    let mut copied = use_signal(|| false);

    let mut is_own_sig = use_signal(|| is_own);
    let mut is_logged_in_sig = use_signal(|| is_logged_in);
    let mut ap_id_sig = use_signal(|| ap_id.clone());

    if *is_own_sig.peek() != is_own {
        is_own_sig.set(is_own);
    }
    if *is_logged_in_sig.peek() != is_logged_in {
        is_logged_in_sig.set(is_logged_in);
    }
    if *ap_id_sig.peek() != ap_id {
        ap_id_sig.set(ap_id.clone());
    }

    let token_check = token.clone();
    let mut follow_resource = use_resource(move || {
        let logged_in = is_logged_in_sig();
        let own = is_own_sig();
        let token = token_check.clone();
        let ap_id = ap_id_sig();
        async move {
            if logged_in && !own {
                if let Some(f) = check_following_fn {
                    f.call(TokenApIdArgs { token, ap_id }).await.ok()
                } else {
                    None
                }
            } else {
                None
            }
        }
    });

    if has_check_fn && !*in_flight.peek() {
        if let Some(resource_val) = follow_resource.read().as_ref() {
            if *follow_status.peek() != *resource_val {
                follow_status.set(*resource_val);
            }
        }
    }

    let token_follow = token.clone();
    let ap_id_follow = ap_id.clone();
    let on_follow = move |_: Event<MouseData>| {
        let token = token_follow.clone();
        let handle_or_url = ap_id_follow.clone();
        in_flight.set(true);
        follow_error.set(None);
        spawn(async move {
            match follow_person_fn.call(FollowArgs { token, handle_or_url }).await {
                Ok(()) => {
                    follow_status.set(Some(Some(false)));
                    if has_check_fn {
                        follow_resource.restart();
                    }
                    if let Some(cb) = on_followed {
                        cb.call(());
                    }
                }
                Err(e) => follow_error.set(Some(e)),
            }
            in_flight.set(false);
        });
    };

    let token_unfollow = token.clone();
    let ap_id_unfollow = ap_id.clone();
    let on_unfollow = move |_: Event<MouseData>| {
        let token = token_unfollow.clone();
        let ap_id = ap_id_unfollow.clone();
        in_flight.set(true);
        follow_error.set(None);
        spawn(async move {
            match unfollow_actor_fn.call(TokenApIdArgs { token, ap_id }).await {
                Ok(()) => {
                    follow_status.set(Some(None));
                    if has_check_fn {
                        follow_resource.restart();
                    }
                }
                Err(e) => follow_error.set(Some(e)),
            }
            in_flight.set(false);
        });
    };

    let copy_handle = handle.clone();
    let on_copy_handle = move |_: Event<MouseData>| {
        let h = copy_handle.clone();
        spawn(async move {
            if copy_to_clipboard(&h).await.is_ok() {
                copied.set(true);
                sleep_ms(1_500).await;
                copied.set(false);
            }
        });
    };

    rsx! {
        if !*is_own_sig.read() {
            if *is_logged_in_sig.read() {
                match *follow_status.read() {
                    None => rsx! {
                        button { class: "btn btn-ghost", disabled: true, "…" }
                    },
                    Some(None) => rsx! {
                        button {
                            class: "btn btn-primary",
                            disabled: *in_flight.read(),
                            onclick: on_follow,
                            if *in_flight.read() { "Following…" } else { "Follow" }
                        }
                    },
                    Some(Some(false)) => rsx! {
                        button {
                            class: format!("btn btn-ghost {}", Styles::follow_pending_btn),
                            disabled: *in_flight.read(),
                            onclick: on_unfollow,
                            if *in_flight.read() { "…" } else { "Pending…" }
                        }
                    },
                    Some(Some(true)) => rsx! {
                        button {
                            class: "btn btn-ghost",
                            disabled: *in_flight.read(),
                            onclick: on_unfollow,
                            if *in_flight.read() { "Unfollowing…" } else { "Unfollow" }
                        }
                    },
                }
            } else {
                button {
                    class: if *copied.read() { "btn btn-ghost" } else { "btn btn-primary" },
                    onclick: on_copy_handle,
                    if *copied.read() { "✓ Copied!" } else { "Follow" }
                }
            }
            if let Some(err) = follow_error.read().clone() {
                ErrorBanner { message: err }
            }
        }
    }
}

#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::error_banner::ErrorBanner;

/// Registration form. No card shell — layout is caller's responsibility.
#[component]
pub fn RegisterPage(
    on_register: EventHandler<(String, String)>,
    on_signin_link: EventHandler<()>,
    loading: bool,
    error: Option<String>,
    #[props(default = true)] show_login_link: bool,
) -> Element {
    let mut username = use_signal(String::new);
    let mut contact = use_signal(String::new);

    rsx! {
        if let Some(ref err) = error {
            ErrorBanner { message: err.clone() }
        }
        div { class: "form-group",
            label { r#for: "username", "Username" }
            input {
                id: "username",
                r#type: "text",
                placeholder: "coolrunner42",
                autocomplete: "username",
                value: "{username}",
                oninput: move |e| username.set(e.value()),
            }
            span { class: "form-hint", "Letters, numbers, underscores. 1–30 chars." }
        }
        div { class: "form-group",
            label { r#for: "contact", "Email or phone" }
            input {
                id: "contact",
                r#type: "text",
                placeholder: "you@example.com or +1234567890",
                autocomplete: "email",
                value: "{contact}",
                oninput: move |e| contact.set(e.value()),
            }
            span { class: "form-hint", "Used for verification only." }
        }
        button {
            class: "btn btn-primary btn-full",
            disabled: loading,
            onclick: move |_| {
                let u = username.read().clone();
                let c = contact.read().clone();
                on_register.call((u, c));
            },
            if loading { "Sending code…" } else { "Send verification code" }
        }
        if show_login_link {
            div { class: "auth-footer",
                "Already have an account? "
                button {
                    class: "btn-link",
                    onclick: move |_| on_signin_link.call(()),
                    "Sign in"
                }
            }
        }
    }
}

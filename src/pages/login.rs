#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::error_banner::ErrorBanner;

#[derive(Clone, PartialEq)]
enum View {
    SignIn,
    ForgotContact,
}

/// Sign-in + forgot-password form. No card shell — layout is caller's responsibility.
/// All navigation is via event handlers so consumers control routing.
#[component]
pub fn LoginPage(
    on_signin: EventHandler<(String, String)>,
    on_reset: EventHandler<String>,
    on_register: EventHandler<()>,
    loading: bool,
    error: Option<String>,
    #[props(default = true)] show_register_link: bool,
) -> Element {
    let mut view = use_signal(|| View::SignIn);
    let mut login_val = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut reset_contact = use_signal(String::new);

    rsx! {
        if let Some(ref err) = error {
            ErrorBanner { message: err.clone() }
        }
        match view.read().clone() {
            View::SignIn => rsx! {
                div { class: "form-group",
                    label { r#for: "login-field", "Username, email, or phone" }
                    input {
                        id: "login-field",
                        r#type: "text",
                        placeholder: "username, you@example.com, or +1234567890",
                        autocomplete: "username",
                        value: "{login_val}",
                        oninput: move |e| login_val.set(e.value()),
                    }
                }
                div { class: "form-group",
                    label { r#for: "password", "Password" }
                    input {
                        id: "password",
                        r#type: "password",
                        placeholder: "••••••••",
                        autocomplete: "current-password",
                        value: "{password}",
                        oninput: move |e| password.set(e.value()),
                    }
                }
                button {
                    class: "btn btn-primary btn-full",
                    disabled: loading,
                    onclick: move |_| {
                        let l = login_val.read().clone();
                        let p = password.read().clone();
                        on_signin.call((l, p));
                    },
                    if loading { "Signing in…" } else { "Sign in" }
                }
                if show_register_link {
                    div { class: "auth-footer",
                        "Don't have an account? "
                        button {
                            class: "btn-link",
                            onclick: move |_| on_register.call(()),
                            "Create one"
                        }
                    }
                }
                div { class: "auth-footer",
                    button {
                        class: "btn-link",
                        onclick: move |_| {
                            reset_contact.set(String::new());
                            view.set(View::ForgotContact);
                        },
                        "Forgot password?"
                    }
                }
            },
            View::ForgotContact => rsx! {
                p { class: "auth-hint",
                    "Enter the email or phone number linked to your account. We'll send you a one-time code."
                }
                div { class: "form-group",
                    label { r#for: "reset-contact", "Email or phone" }
                    input {
                        id: "reset-contact",
                        r#type: "text",
                        placeholder: "you@example.com",
                        autocomplete: "email",
                        value: "{reset_contact}",
                        oninput: move |e| reset_contact.set(e.value()),
                    }
                }
                button {
                    class: "btn btn-primary btn-full",
                    disabled: loading || reset_contact.read().trim().is_empty(),
                    onclick: move |_| {
                        let c = reset_contact.read().clone();
                        on_reset.call(c);
                    },
                    if loading { "Sending…" } else { "Send code" }
                }
                div { class: "auth-footer",
                    button {
                        class: "btn-link",
                        onclick: move |_| view.set(View::SignIn),
                        "Back to sign in"
                    }
                }
            },
        }
    }
}

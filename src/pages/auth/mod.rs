use dioxus::prelude::*;

use crate::components::{error_banner::ErrorBanner, otp_password_form::OtpPasswordForm};

#[component]
pub fn ResetPasswordShell(
    owner_username: Option<String>,
    primary_error: Option<String>,
    show_rerequest: bool,
    invalid_otp: bool,
    rerequest_contact: Signal<String>,
    rerequest_loading: bool,
    rerequest_sent: bool,
    rerequest_error: Option<String>,
    on_rerequest: EventHandler<Event<MouseData>>,
    otp: Signal<String>,
    password: Signal<String>,
    password2: Signal<String>,
    loading: bool,
    on_submit: EventHandler<Event<MouseData>>,
    #[props(optional)] on_verify_otp: Option<EventHandler<String>>,
    #[props(optional)] otp_verify_error: Option<String>,
    #[props(default)] otp_pre_verified: bool,
) -> Element {
    rsx! {
        if let Some(u) = owner_username {
            p { class: "auth-instance", "Dedicated server for @{u}" }
        }
        if let Some(err) = primary_error {
            ErrorBanner { message: err }
        }
        if show_rerequest {
            if invalid_otp {
                p { class: "auth-hint otp-expired-hint",
                    "This verification link has expired or has already been used."
                }
            } else {
                p { class: "auth-hint",
                    "No reset code found. Enter your email or phone to receive a new link."
                }
            }

            if rerequest_sent {
                p { class: "auth-hint auth-hint-success",
                    "Reset link sent — check your inbox and follow the link."
                }
            } else {
                if let Some(err) = rerequest_error {
                    ErrorBanner { message: err }
                }
                div { class: "auth-field",
                    label { r#for: "rerequest-contact", "Email or phone" }
                    input {
                        id: "rerequest-contact",
                        r#type: "text",
                        autocomplete: "email",
                        placeholder: "you@example.com",
                        value: "{rerequest_contact}",
                        oninput: move |e| rerequest_contact.set(e.value()),
                    }
                }
                button {
                    class: "btn btn-primary btn-full",
                    disabled: rerequest_loading,
                    onclick: on_rerequest,
                    if rerequest_loading { "Sending…" } else { "Send reset link" }
                }
            }

            div { class: "auth-footer",
                Link { to: "/login", "Back to sign in" }
            }
        } else {
            OtpPasswordForm {
                otp,
                password,
                password2,
                loading,
                on_submit,
                submit_label: "Continue".to_string(),
                loading_label: "Verifying…".to_string(),
                password_label: "Password".to_string(),
                password2_label: "Confirm password".to_string(),
                on_verify_otp,
                otp_verify_error,
                otp_pre_verified,
            }
            div { class: "auth-footer",
                Link { to: "/login", "Back to sign in" }
            }
        }
    }
}

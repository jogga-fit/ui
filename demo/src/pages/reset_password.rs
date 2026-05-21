#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::{
    components::{error_banner::ErrorBanner, otp_password_form::OtpPasswordForm},
    sleep_ms,
};

#[component]
pub fn ResetPasswordDemoPage() -> Element {
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);
    let nav = use_navigator();

    let on_submit = move |(pwd, pwd2): (String, String)| {
        if pwd != pwd2 {
            error.set(Some("Passwords don't match.".into()));
            return;
        }
        if pwd.len() < 8 {
            error.set(Some("Password must be at least 8 characters.".into()));
            return;
        }
        loading.set(true);
        error.set(None);
        spawn(async move {
            sleep_ms(800).await;
            nav.push("/feed");
            loading.set(false);
        });
    };

    rsx! {
        if let Some(ref err) = *error.read() {
            ErrorBanner { message: err.clone() }
        }
        OtpPasswordForm {
            loading: *loading.read(),
            on_submit,
            submit_label: "Continue".to_string(),
            loading_label: "Verifying…".to_string(),
            password_label: "Password".to_string(),
            password2_label: "Confirm password".to_string(),
        }
        div { class: "auth-footer",
            a { href: "/login", "Back to sign in" }
        }
    }
}

#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::{
    pages::LoginPage as LoginPageShell,
    types::{AuthSignal, AuthUser, sleep_ms},
};

#[component]
pub fn LoginPage() -> Element {
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);
    let auth = use_context::<AuthSignal>();
    let nav = use_navigator();

    let on_signin = move |(login, password): (String, String)| {
        if login.is_empty() || password.is_empty() {
            error.set(Some("Please fill in all fields.".into()));
            return;
        }
        loading.set(true);
        error.set(None);
        spawn(async move {
            sleep_ms(800).await;
            let username = "alex";
            let token = "demo-token";
            let cookie_js = format!(
                "document.cookie='jogga_auth={}:{}; path=/; max-age=2592000; SameSite=Lax';",
                username, token
            );
            let _ = document::eval(&cookie_js).await;
            auth.clone().set(Some(AuthUser {
                token: token.to_string(),
                username: username.to_string(),
                ap_id: "https://jogga.fit/users/alex".to_string(),
            }));
            nav.push("/feed");
            loading.set(false);
        });
    };

    let on_reset = move |_contact: String| {
        nav.push("/reset-password");
    };

    let on_register = move |_: ()| {
        nav.push("/register");
    };

    rsx! {
        LoginPageShell {
            on_signin,
            on_reset,
            on_register,
            loading: *loading.read(),
            error: error.read().clone(),
        }
    }
}

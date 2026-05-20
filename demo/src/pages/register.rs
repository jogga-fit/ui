#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::{pages::RegisterPage as RegisterPageShell, sleep_ms};

#[component]
pub fn RegisterDemoPage() -> Element {
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);
    let nav = use_navigator();

    let on_register = move |(username, contact): (String, String)| {
        let _ = (username, contact);
        loading.set(true);
        error.set(None);
        spawn(async move {
            sleep_ms(800).await;
            nav.push("/reset-password");
            loading.set(false);
        });
    };

    let on_signin_link = move |_: ()| {
        nav.push("/login");
    };

    rsx! {
        RegisterPageShell {
            on_register,
            on_signin_link,
            loading: *loading.read(),
            error: error.read().clone(),
        }
    }
}

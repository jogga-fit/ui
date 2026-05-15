#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::pages::SettingsPageView;

use crate::mock::{mock_me, settings_fns};

#[component]
pub fn SettingsPage() -> Element {
    let (set_theme_fn, set_privacy_settings_fn, delete_account_fn, ..) = settings_fns();

    rsx! {
        SettingsPageView {
            profile: Some(Ok(mock_me())),
            set_theme_fn,
            set_privacy_settings_fn,
            delete_account_fn,
            on_theme_saved: move |_| {},
            on_account_deleted: move |_| {},
        }
    }
}

#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::pages::SettingsPageView;

use crate::mock::{mock_delete_account, mock_me, mock_set_privacy_settings, mock_set_theme};

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        SettingsPageView {
            profile: Some(Ok(mock_me())),
            set_theme_fn: Callback::new(mock_set_theme),
            set_privacy_settings_fn: Callback::new(mock_set_privacy_settings),
            delete_account_fn: Callback::new(mock_delete_account),
            on_theme_saved: move |_| {},
            on_account_deleted: move |_| {},
        }
    }
}

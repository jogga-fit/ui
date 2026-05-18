#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::error_banner::ErrorBanner;

#[css_module("/src/components/auth_card/style.css")]
struct Styles;

#[component]
pub fn AuthCard(
    subtitle: String,
    #[props(optional)] error: Option<String>,
    #[props(optional)] instance_label: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        div { class: Styles::auth_page, "data-testid": "auth-page",
            div { class: Styles::auth_card, "data-testid": "auth-card",
                div { class: Styles::auth_header,
                    div { class: Styles::auth_logo, i { class: "ph ph-person-simple-run" } }
                    h1 { "Jogga" }
                    if let Some(label) = instance_label {
                        p { class: "auth-instance", "{label}" }
                    }
                    p { class: Styles::auth_subtitle, "{subtitle}" }
                }
                if let Some(err) = error {
                    ErrorBanner { message: err }
                }
                {children}
            }
        }
    }
}

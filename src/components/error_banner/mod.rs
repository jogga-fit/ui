use dioxus::prelude::*;

#[css_module("/src/components/error_banner/style.css")]
struct Styles;

#[component]
pub fn ErrorBanner(message: String) -> Element {
    rsx! {
        div { class: "error-banner {Styles::error_banner}", "{message}" }
    }
}

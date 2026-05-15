use dioxus::prelude::*;

#[css_module("/src/components/empty_state/style.css")]
struct Styles;

#[component]
pub fn EmptyState(icon: String, title: String, children: Element) -> Element {
    rsx! {
        div { class: "{Styles::empty_state} empty-state",
            div { class: "{Styles::empty_icon} empty-icon", i { class: "{icon}" } }
            h3 { "{title}" }
            {children}
        }
    }
}

use dioxus::prelude::*;

#[css_module("/src/components/empty_state/style.css")]
struct Styles;

#[component]
pub fn EmptyState(icon: String, title: String, children: Element) -> Element {
    rsx! {
        div { class: Styles::empty_state,
            div { class: Styles::empty_icon, i { class: "{icon}" } }
            h3 { "{title}" }
            {children}
        }
    }
}

use dioxus::prelude::*;

#[css_module("/src/components/tab_bar/style.css")]
struct Styles;

#[component]
pub fn TabBar(children: Element) -> Element {
    rsx! {
        div { class: Styles::tab_bar, {children} }
    }
}

#[component]
pub fn TabBtn(
    label: String,
    active: bool,
    onclick: EventHandler<()>,
    #[props(default)] badge: Option<usize>,
    #[props(default)] children: Element,
) -> Element {
    rsx! {
        button {
            class: "{Styles::tab_btn}",
            class: if active { "{Styles::tab_btn_active}" },
            onclick: move |_| onclick.call(()),
            "{label}"
            if let Some(count) = badge {
                span { class: Styles::tab_badge, "{count}" }
            }
            {children}
        }
    }
}

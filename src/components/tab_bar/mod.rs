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
    let class = if active {
        format!("{} {}", Styles::tab_btn, Styles::tab_btn_active)
    } else {
        Styles::tab_btn.to_string()
    };

    rsx! {
        button {
            class,
            onclick: move |_| onclick.call(()),
            "{label}"
            if let Some(count) = badge {
                span { class: Styles::tab_badge, "{count}" }
            }
            {children}
        }
    }
}

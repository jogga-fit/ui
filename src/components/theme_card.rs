#![allow(non_snake_case)]

use dioxus::prelude::*;

#[css_module("/src/components/theme_card.css")]
struct Styles;

#[component]
pub fn ThemeCard(
    id: &'static str,
    label: &'static str,
    active: bool,
    disabled: bool,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: if active {
                format!("{} {}", Styles::theme_card, Styles::theme_card_active)
            } else {
                Styles::theme_card.to_string()
            },
            "data-testid": "theme-option-{id}",
            "aria-pressed": if active { "true" } else { "false" },
            disabled,
            onclick: move |e| onclick.call(e),
            {children}
            span { class: Styles::theme_card_label,
                if active {
                    i { class: "ph ph-check-circle", style: "color: var(--primary); margin-right: 4px;" }
                }
                "{label}"
            }
        }
    }
}

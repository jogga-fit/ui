use dioxus::prelude::*;

#[css_module("/src/components/setting_toggle/style.css")]
struct Styles;

#[component]
pub fn SettingToggle(
    label: String,
    description: String,
    checked: bool,
    disabled: bool,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        div { class: Styles::toggle_row,
            div { class: Styles::toggle_info,
                span { class: Styles::toggle_label, "{label}" }
                span { class: Styles::toggle_desc, "{description}" }
            }
            label { class: Styles::toggle_switch,
                input {
                    r#type: "checkbox",
                    "aria-label": "{label}",
                    checked,
                    disabled,
                    onchange: move |e| onchange.call(e.checked()),
                }
                span { class: Styles::toggle_thumb }
            }
        }
    }
}

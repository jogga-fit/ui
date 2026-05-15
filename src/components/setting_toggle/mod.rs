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
        div { class: "{Styles::toggle_row} toggle-row",
            div { class: "{Styles::toggle_info} toggle-info",
                span { class: "{Styles::toggle_label} toggle-label", "{label}" }
                span { class: "{Styles::toggle_desc} toggle-desc", "{description}" }
            }
            label { class: "{Styles::toggle_switch} toggle-switch",
                input {
                    r#type: "checkbox",
                    checked,
                    disabled,
                    onchange: move |e| onchange.call(e.checked()),
                }
                span { class: "{Styles::toggle_thumb} toggle-thumb" }
            }
        }
    }
}

use dioxus::prelude::*;

use crate::components::error_banner::ErrorBanner;

#[css_module("/src/components/remote_follow_card/style.css")]
struct Styles;

#[component]
pub fn RemoteFollowCard(
    icon_class: String,
    title: String,
    description: String,
    input_label: String,
    placeholder: String,
    value: String,
    examples: Vec<String>,
    button_icon: String,
    button_label: String,
    busy_label: String,
    busy: bool,
    disabled: bool,
    error: Option<String>,
    success_message: Option<String>,
    on_input: EventHandler<String>,
    on_submit: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "{Styles::remote_follow_card} follow-card",
            div { class: Styles::remote_follow_header,
                div { class: Styles::remote_follow_icon,
                    i { class: "{icon_class}" }
                }
                h2 { class: Styles::remote_follow_title, "{title}" }
                div { class: Styles::remote_follow_help,
                    button {
                        class: Styles::remote_follow_help_btn,
                        r#type: "button",
                        aria_label: "Help for {title}",
                        i { class: "ph ph-info" }
                    }
                    div { class: Styles::remote_follow_tooltip,
                        p { "{description}" }
                        if !examples.is_empty() {
                            div { class: Styles::remote_follow_examples,
                                span { "Examples" }
                                for example in examples.iter() {
                                    code { key: "{example}", "{example}" }
                                }
                            }
                        }
                    }
                }
            }

            div { class: Styles::remote_follow_form,
                input {
                    class: "{Styles::remote_follow_input} follow-input",
                    r#type: "text",
                    aria_label: "{input_label}",
                    placeholder: "{placeholder}",
                    value: "{value}",
                    oninput: move |e| on_input.call(e.value()),
                }
                button {
                    class: Styles::remote_follow_submit,
                    disabled: busy || disabled,
                    onclick: move |_| on_submit.call(()),
                    i { class: "{button_icon}" }
                    span { if busy { "{busy_label}" } else { "{button_label}" } }
                }
            }

            if let Some(err) = error {
                ErrorBanner { message: err }
            }

            if let Some(message) = success_message {
                div { class: Styles::remote_follow_success,
                    i { class: "ph ph-check-circle" }
                    "{message}"
                }
            }
        }
    }
}

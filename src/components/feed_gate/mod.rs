use dioxus::prelude::*;

#[css_module("/src/components/feed_gate/style.css")]
struct Styles;

/// Renders `content` blurred behind a gradient overlay with a CTA card.
/// Pass sign-in/register buttons as `children`.
#[component]
pub fn FeedGate(
    icon: String,
    title: String,
    description: String,
    content: Element,
    children: Element,
) -> Element {
    rsx! {
        div { class: Styles::feed_gate,
            div { class: Styles::feed_gate_blur, {content} }
            div { class: Styles::feed_gate_overlay,
                div { class: Styles::feed_gate_card,
                    i { class: "{icon} {Styles::feed_gate_icon}" }
                    h3 { class: Styles::feed_gate_title, "{title}" }
                    p { class: Styles::feed_gate_desc, "{description}" }
                    div { class: Styles::feed_gate_actions, {children} }
                }
            }
        }
    }
}

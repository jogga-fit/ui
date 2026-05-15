use dioxus::prelude::*;

#[css_module("/src/components/avatar/style.css")]
struct Styles;

#[component]
pub fn Avatar(url: Option<String>, name: String, #[props(default)] size: String) -> Element {
    let size_classes = match size.as_str() {
        "" => String::new(),
        "avatar-sm" => format!(" avatar-sm {}", Styles::avatar_sm),
        "avatar-lg" => format!(" avatar-lg {}", Styles::avatar_lg),
        other => format!(" {other}"),
    };
    let avatar_class = format!("{} avatar{size_classes}", Styles::avatar);
    let initial = name
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();

    rsx! {
        if let Some(url) = url {
            img {
                class: format!("{avatar_class} {} avatar-img", Styles::avatar_img),
                src: "{url}",
                alt: "{name}",
            }
        } else {
            div { class: avatar_class, "{initial}" }
        }
    }
}

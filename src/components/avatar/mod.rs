use dioxus::prelude::*;

#[css_module("/src/components/avatar/style.css")]
struct Styles;

#[derive(Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum AvatarSize {
    #[default]
    Small,
    Medium,
    Large,
}

#[component]
pub fn Avatar(url: Option<String>, name: String, #[props(default)] size: AvatarSize) -> Element {
    let size_class = match size {
        AvatarSize::Small => Styles::avatar_sm,
        AvatarSize::Medium => Styles::avatar_md,
        AvatarSize::Large => Styles::avatar_lg,
    };
    let initial = name
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();

    rsx! {
        if let Some(url) = url {
            img {
                class: "{Styles::avatar} {size_class} {Styles::avatar_img}",
                src: "{url}",
                alt: "{name}",
            }
        } else {
            div { class: "{Styles::avatar} {size_class}", "{initial}" }
        }
    }
}

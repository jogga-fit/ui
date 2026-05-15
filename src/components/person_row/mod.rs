use dioxus::prelude::*;

use crate::components::avatar::Avatar;

#[css_module("/src/components/person_row/style.css")]
struct Styles;

#[component]
pub fn PersonRow(
    username: String,
    domain: String,
    #[props(default)] display_name: Option<String>,
    #[props(default)] avatar_url: Option<String>,
    is_local: bool,
    #[props(default)] bio: Option<String>,
    #[props(default)] children: Element,
) -> Element {
    let name = display_name.as_deref().unwrap_or(&username).to_string();
    let handle = if is_local {
        format!("@{username}")
    } else {
        format!("@{username}@{domain}")
    };

    rsx! {
        div { class: Styles::person_row,
            div {
                class: if is_local {
                    Styles::person_row_identity.to_string()
                } else {
                    format!("{} {}", Styles::person_row_identity, Styles::person_row_identity_remote)
                },
                Avatar { url: avatar_url, name: name.clone(), size: "36px" }
                div { class: Styles::person_row_info,
                    span { class: Styles::person_row_name, "{name}" }
                    span { class: Styles::person_row_handle, "{handle}" }
                    if let Some(bio_text) = bio {
                        span { class: Styles::person_row_bio, "{bio_text}" }
                    }
                }
            }
            {children}
        }
    }
}

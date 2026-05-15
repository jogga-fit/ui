use dioxus::prelude::*;

use crate::components::avatar::Avatar;

#[css_module("/src/components/actor_link/style.css")]
struct Styles;

#[component]
pub fn ActorLink(
    is_local: bool,
    ap_id: String,
    username: String,
    #[props(default)] display_name: Option<String>,
    domain: String,
    avatar_url: Option<String>,
    profile_href: String,
    #[props(default)] club_href: Option<String>,
    #[props(default)] via_club_display: Option<String>,
    #[props(default)] stop_propagation: bool,
) -> Element {
    let label = display_name.unwrap_or_else(|| username.clone());

    let href = if is_local { profile_href } else { ap_id };
    let target = if is_local { "_self" } else { "_blank" };
    let rel = if is_local { "" } else { "noopener noreferrer" };

    rsx! {
        a {
            class: "{Styles::feed_avatar_wrap} {Styles::feed_actor_link}",
            href: "{href}",
            target: "{target}",
            rel: "{rel}",
            onclick: move |e: MouseEvent| {
                if stop_propagation {
                    e.stop_propagation();
                }
            },

            Avatar { url: avatar_url, name: label.clone() }

            span { class: Styles::feed_username,
                "{label}"

                // Render club link if both href and display name exist
                if let (Some(ch), Some(cd)) = (club_href, via_club_display) {
                    span { class: Styles::feed_via_club,
                        i { class: "ph ph-caret-right" },
                        a {
                            class: Styles::feed_club_link,
                            href: "{ch}",
                            onclick: move |e: MouseEvent| e.stop_propagation(),
                            "{cd}"
                        }
                    }
                }
            }
        }
    }
}

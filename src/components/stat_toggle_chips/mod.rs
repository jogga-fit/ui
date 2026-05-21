use dioxus::prelude::*;

#[css_module("/src/components/stat_toggle_chips/style.css")]
pub struct Styles;

pub const STAT_TOGGLES: &[(&str, &str)] = &[
    ("avg_heart_rate_bpm", "Heart rate"),
    ("max_heart_rate_bpm", "Max HR"),
    ("avg_power_w", "Avg power"),
    ("max_power_w", "Max power"),
    ("normalized_power_w", "NP"),
    ("avg_cadence_rpm", "Cadence"),
];

const MAP_KEY: &str = "map";

#[component]
pub fn StatToggleChips(
    has_map: bool,
    stats: Vec<(String, String)>,
    hidden_stats: Vec<String>,
    on_toggle: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "compose-field-row",
            label { class: "compose-label", "Hide from post" }
            div { class: Styles::type_chip_row,
                if has_map {
                    {
                        let map_hidden = hidden_stats.contains(&MAP_KEY.to_string());
                        rsx! {
                            button {
                                r#type: "button",
                                class: "{Styles::type_chip}",
                                class: if map_hidden { "{Styles::type_chip_active}" },
                                onclick: move |_| on_toggle.call(MAP_KEY.to_string()),
                                i { class: "ph ph-map-trifold" }
                                " Map"
                            }
                        }
                    }
                }
                {stats.into_iter().map(|(key, label)| {
                    let is_hidden = hidden_stats.contains(&key);
                    rsx! {
                        button {
                            key: "{key}",
                            r#type: "button",
                            class: "{Styles::type_chip}",
                            class: if is_hidden { "{Styles::type_chip_active}" },
                            onclick: move |_| on_toggle.call(key.clone()),
                            "{label}"
                        }
                    }
                })}
            }
        }
    }
}

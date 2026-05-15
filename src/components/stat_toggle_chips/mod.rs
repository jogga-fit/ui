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
    hidden_stats: Signal<Vec<String>>,
) -> Element {
    rsx! {
        div { class: "compose-field-row",
            label { class: "compose-label", "Hide from post" }
            div { class: Styles::type_chip_row,
                if has_map {
                    {
                        let map_hidden = hidden_stats.read().contains(&MAP_KEY.to_string());
                        rsx! {
                            button {
                                r#type: "button",
                                class: "{Styles::type_chip}",
                                class: if map_hidden { "{Styles::type_chip_active}" },
                                onclick: move |_| {
                                    let mut hs = hidden_stats.write();
                                    let k = MAP_KEY.to_string();
                                    if hs.contains(&k) {
                                        hs.retain(|s| s != &k);
                                    } else {
                                        hs.push(k);
                                    }
                                },
                                i { class: "ph ph-map-trifold" }
                                " Map"
                            }
                        }
                    }
                }
                {stats.into_iter().map(|(key, label)| {
                    let k = key.clone();
                    let k2 = key.clone();
                    rsx! {
                        button {
                            key: "{k2}",
                            r#type: "button",
                            class: if hidden_stats.read().contains(&k) {
                                format!("{} {}", Styles::type_chip, Styles::type_chip_active)
                            } else {
                                Styles::type_chip.to_string()
                            },
                            onclick: move |_| {
                                let mut hs = hidden_stats.write();
                                if hs.contains(&k2) {
                                    hs.retain(|s| s != &k2);
                                } else {
                                    hs.push(k2.clone());
                                }
                            },
                            "{label}"
                        }
                    }
                })}
            }
        }
    }
}

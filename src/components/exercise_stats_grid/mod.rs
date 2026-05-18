use dioxus::prelude::*;

use crate::format::{fmt_distance, fmt_duration, fmt_elevation, fmt_pace};

#[css_module("/src/components/exercise_stats_grid/style.css")]
struct Styles;

#[component]
pub fn ExerciseStatsGrid(
    #[props(default)] distance_m: Option<f64>,
    #[props(default)] duration_s: Option<i64>,
    #[props(default)] pace_s_per_km: Option<f64>,
    #[props(default)] elevation_gain_m: Option<f64>,
    #[props(default)] avg_heart_rate_bpm: Option<i32>,
    #[props(default)] max_heart_rate_bpm: Option<i32>,
    #[props(default)] avg_power_w: Option<f64>,
    #[props(default)] max_power_w: Option<f64>,
    #[props(default)] normalized_power_w: Option<f64>,
    #[props(default)] avg_cadence_rpm: Option<f64>,
    #[props(default)] extra_class: String,
) -> Element {
    let class = if extra_class.is_empty() {
        Styles::stats_grid.to_string()
    } else {
        format!("{} {extra_class}", Styles::stats_grid)
    };

    rsx! {
        div { class: "{class}",
            if let Some(d) = distance_m {
                if d > 0.0 {
                    div { class: Styles::stat_cell,
                        span { class: Styles::stat_value, "{fmt_distance(d)}" }
                        span { class: Styles::stat_label, "Distance" }
                    }
                }
            }
            if let Some(dur) = duration_s {
                if dur > 0 {
                    div { class: Styles::stat_cell,
                        span { class: Styles::stat_value, "{fmt_duration(dur as i32)}" }
                        span { class: Styles::stat_label, "Time" }
                    }
                }
            }
            if let Some(p) = pace_s_per_km {
                if p > 0.0 {
                    div { class: Styles::stat_cell,
                        span { class: Styles::stat_value, "{fmt_pace(p)}" }
                        span { class: Styles::stat_label, "Pace" }
                    }
                }
            }
            if let Some(e) = elevation_gain_m {
                if e > 0.0 {
                    div { class: Styles::stat_cell,
                        span { class: Styles::stat_value, "{fmt_elevation(e)}" }
                        span { class: Styles::stat_label, "Elevation" }
                    }
                }
            }
            if let Some(hr) = avg_heart_rate_bpm {
                div { class: Styles::stat_cell,
                    span { class: Styles::stat_value, "{hr} bpm" }
                    span { class: Styles::stat_label, "Avg HR" }
                }
            }
            if let Some(hr) = max_heart_rate_bpm {
                div { class: Styles::stat_cell,
                    span { class: Styles::stat_value, "{hr} bpm" }
                    span { class: Styles::stat_label, "Max HR" }
                }
            }
            if let Some(pwr) = avg_power_w {
                div { class: Styles::stat_cell,
                    span { class: Styles::stat_value, "{pwr:.0} W" }
                    span { class: Styles::stat_label, "Avg Power" }
                }
            }
            if let Some(pwr) = max_power_w {
                div { class: Styles::stat_cell,
                    span { class: Styles::stat_value, "{pwr:.0} W" }
                    span { class: Styles::stat_label, "Max Power" }
                }
            }
            if let Some(np) = normalized_power_w {
                div { class: Styles::stat_cell,
                    span { class: Styles::stat_value, "{np:.0} W" }
                    span { class: Styles::stat_label, "NP" }
                }
            }
            if let Some(cad) = avg_cadence_rpm {
                div { class: Styles::stat_cell,
                    span { class: Styles::stat_value, "{cad:.0} rpm" }
                    span { class: Styles::stat_label, "Cadence" }
                }
            }
        }
    }
}

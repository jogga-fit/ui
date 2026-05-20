use dioxus::prelude::*;
use dioxus_leaflet::{
    FitBoundsOptions, LatLng, LatLngBounds, Map, MapFitBounds, MapOptions, MapPosition,
    PathOptions, Point, Polyline, TileLayer,
};

use crate::{FetchRouteArgs, RouteFn, components::skeleton::Skeleton};

#[css_module("/src/components/route_map/style.css")]
struct Styles;

fn elevation_polyline(coords: &[(f64, f64, Option<f64>)], max_pts: usize) -> Option<String> {
    let elevations: Vec<f64> = coords.iter().filter_map(|(_, _, e)| *e).collect();
    if elevations.len() < 2 {
        return None;
    }
    let step = elevations.len().div_ceil(max_pts);
    let sampled: Vec<f64> = elevations.iter().step_by(step).copied().collect();
    let min_e = sampled.iter().copied().fold(f64::INFINITY, f64::min);
    let max_e = sampled.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = max_e - min_e;
    if range < 1.0 {
        return None;
    }
    let n = sampled.len();
    if n < 2 {
        return None;
    }
    let pts = sampled
        .iter()
        .enumerate()
        .map(|(i, &e)| {
            let x = (i as f64) / (n - 1) as f64 * 200.0;
            let y = 38.0 - ((e - min_e) / range) * 36.0;
            format!("{x:.1},{y:.1}")
        })
        .collect::<Vec<_>>()
        .join(" ");
    Some(pts)
}

fn carto_dark_matter() -> TileLayer {
    TileLayer {
        url: "https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png".to_string(),
        attribution: "&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors &copy; <a href=\"https://carto.com/attributions\">CARTO</a>".to_string(),
        max_zoom: 19,
        subdomains: vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()],
    }
}

fn route_bounds(coords: &[LatLng]) -> Option<(f64, f64, f64, f64)> {
    let mut min_lat = f64::INFINITY;
    let mut max_lat = f64::NEG_INFINITY;
    let mut min_lng = f64::INFINITY;
    let mut max_lng = f64::NEG_INFINITY;

    for coord in coords {
        if !coord.lat.is_finite() || !coord.lng.is_finite() {
            continue;
        }
        min_lat = min_lat.min(coord.lat);
        max_lat = max_lat.max(coord.lat);
        min_lng = min_lng.min(coord.lng);
        max_lng = max_lng.max(coord.lng);
    }

    if !min_lat.is_finite() || !max_lat.is_finite() || !min_lng.is_finite() || !max_lng.is_finite()
    {
        return None;
    }

    Some((min_lat, max_lat, min_lng, max_lng))
}

fn route_zoom(min_lat: f64, max_lat: f64, min_lng: f64, max_lng: f64) -> f64 {
    let span = (max_lat - min_lat).abs().max((max_lng - min_lng).abs());
    if span <= 0.006 {
        15.0
    } else if span <= 0.014 {
        14.0
    } else if span <= 0.035 {
        13.0
    } else if span <= 0.08 {
        12.0
    } else if span <= 0.16 {
        11.0
    } else {
        10.0
    }
}

fn route_center(coords: &[LatLng]) -> MapPosition {
    let Some((min_lat, max_lat, min_lng, max_lng)) = route_bounds(coords) else {
        return MapPosition::new(0.0, 0.0, 13.0);
    };
    MapPosition::new(
        (min_lat + max_lat) / 2.0,
        (min_lng + max_lng) / 2.0,
        route_zoom(min_lat, max_lat, min_lng, max_lng),
    )
}

#[component]
pub fn RouteMap(
    route_url: String,
    token: Option<String>,
    #[props(default = true)] interactive: bool,
    route_fn: RouteFn,
) -> Element {
    let url = route_url.clone();
    let tok = token.clone();
    let route_resource = use_resource(move || {
        let url = url.clone();
        let token = tok.clone();
        async move { route_fn.call(FetchRouteArgs { url, token }).await }
    });

    match route_resource() {
        None => rsx! { Skeleton { style: "height: 200px; border-radius: 8px;" } },
        Some(Err(_)) | Some(Ok(None)) => rsx! {},
        Some(Ok(Some(coords))) if coords.is_empty() => rsx! {},
        Some(Ok(Some(coords))) => {
            let latlngs: Vec<LatLng> = coords
                .iter()
                .map(|&(lat, lon, _)| LatLng::new(lat, lon))
                .collect();
            rsx! { RouteMapFromCoords { coords: latlngs, interactive } }
        }
    }
}

#[component]
pub fn RouteMapFromCoords(
    coords: Vec<LatLng>,
    #[props(default = String::from("200px"))] map_height: String,
    #[props(default = true)] interactive: bool,
) -> Element {
    if coords.is_empty() {
        return rsx! {};
    }
    let center = route_center(&coords);
    rsx! { RouteMapInner { coords, center, height: map_height, interactive } }
}

#[component]
pub fn RouteSection(
    route_url: String,
    token: Option<String>,
    #[props(default = String::from("200px"))] map_height: String,
    #[props(default = true)] interactive: bool,
    #[props(default)] show_elevation: bool,
    route_fn: RouteFn,
) -> Element {
    let url = route_url.clone();
    let tok = token.clone();
    let route_resource = use_resource(move || {
        let url = url.clone();
        let token = tok.clone();
        async move { route_fn.call(FetchRouteArgs { url, token }).await }
    });

    match route_resource() {
        None => rsx! { Skeleton { style: "height: {map_height}; border-radius: 8px;" } },
        Some(Err(_)) | Some(Ok(None)) => rsx! {},
        Some(Ok(Some(coords))) if coords.is_empty() => rsx! {},
        Some(Ok(Some(coords))) => {
            let latlngs: Vec<LatLng> = coords
                .iter()
                .map(|&(lat, lon, _)| LatLng::new(lat, lon))
                .collect();
            rsx! {
                RouteMapFromCoords { coords: latlngs, map_height: map_height.clone(), interactive }
                if show_elevation {
                    ElevationSparkline { coords: coords.clone() }
                }
            }
        }
    }
}

#[component]
fn ElevationSparkline(coords: Vec<(f64, f64, Option<f64>)>) -> Element {
    let Some(pts) = elevation_polyline(&coords, 200) else {
        return rsx! {};
    };
    let first_x = pts
        .split_once(',')
        .map(|(x, _)| x)
        .unwrap_or("0")
        .to_string();
    let last_x = pts
        .rsplit(' ')
        .next()
        .and_then(|p| p.split_once(','))
        .map(|(x, _)| x)
        .unwrap_or("200")
        .to_string();
    let fill_pts = format!("{pts} {last_x},40 {first_x},40");
    rsx! {
        svg {
            class: Styles::elevation_sparkline,
            view_box: "0 0 200 40",
            preserve_aspect_ratio: "none",
            xmlns: "http://www.w3.org/2000/svg",
            polygon { points: "{fill_pts}", class: Styles::sparkline_fill }
            polyline { points: "{pts}", class: Styles::sparkline_line, fill: "none" }
        }
    }
}

#[component]
fn RouteMapInner(
    coords: Vec<LatLng>,
    center: MapPosition,
    height: String,
    interactive: bool,
) -> Element {
    // Gate on client-side mount: SSR has no DOM so Leaflet's JS interop fails,
    // and on the client the div must be committed before use_resource polls.
    let mut ready = use_signal(|| false);
    use_effect(move || {
        ready.set(true);
    });

    let coords_signal = use_signal(|| vec![coords.clone()]);
    let opts_signal = use_signal(|| PathOptions {
        fill: false,
        color: dioxus_leaflet::Color::from_rgb8(0xc1, 0x44, 0x0e),
        fill_color: dioxus_leaflet::Color::from_rgb8(0xc1, 0x44, 0x0e),
        weight: 4,
        opacity: 0.9,
        ..PathOptions::default()
    });
    let map_opts = if interactive {
        MapOptions::default()
    } else {
        MapOptions::minimal()
    };
    let map_opts = map_opts.with_tile_layer(carto_dark_matter());
    // Use container-aware fitBounds so the route fills the map regardless of tile size.
    // 20 px padding gives ≥ 10 px margin after the ceil-to-integer-zoom step in JS.
    let map_opts = if let Some((min_lat, max_lat, min_lng, max_lng)) = route_bounds(&coords) {
        map_opts.with_fit_bounds(
            MapFitBounds::new(LatLngBounds::new(
                LatLng::new(min_lat, min_lng),
                LatLng::new(max_lat, max_lng),
            ))
            .with_options(FitBoundsOptions {
                padding: Some(Point::new(20.0, 20.0)),
                ..Default::default()
            }),
        )
    } else {
        map_opts
    };

    rsx! {
        div { class: Styles::route_map, "data-testid": "route-map",
            if ready() {
                Map {
                    initial_position: center,
                    height: "{height}",
                    options: map_opts,
                    Polyline {
                        coordinates: coords_signal.boxed(),
                        options: opts_signal.boxed(),
                    }
                }
            } else {
                div { style: "height: {height}" }
            }
        }
    }
}

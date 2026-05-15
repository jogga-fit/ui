use dioxus::prelude::*;

use crate::{RouteFn, components::route_map::RouteSection};

#[css_module("/src/components/media_carousel/style.css")]
struct Styles;

#[component]
pub fn MediaCollage(
    route_url: Option<String>,
    image_urls: Vec<String>,
    token: Option<String>,
    #[props(default = false)] interactive: bool,
    #[props(default = String::from("240px"))] map_height: String,
    #[props(default)] on_open_overlay: Option<EventHandler<usize>>,
    route_fn: RouteFn,
) -> Element {
    let has_map = route_url.is_some();
    let img_count = image_urls.len();

    if !has_map && img_count == 0 {
        return rsx! {};
    }

    if interactive {
        return rsx! {
            DetailLayout { route_url, image_urls, token, map_height, route_fn }
        };
    }

    if on_open_overlay.is_some() {
        return rsx! {
            FeedCollage {
                route_url,
                image_urls,
                token,
                map_height,
                route_fn,
                on_open: move |i| { if let Some(h) = on_open_overlay.as_ref() { h.call(i); } },
            }
        };
    }

    let mut carousel_idx: Signal<Option<usize>> = use_signal(|| None);

    rsx! {
        FeedCollage {
            route_url: route_url.clone(),
            image_urls: image_urls.clone(),
            token: token.clone(),
            map_height,
            route_fn,
            on_open: move |i| carousel_idx.set(Some(i)),
        }
        if let Some(idx) = *carousel_idx.read() {
            CarouselOverlay {
                route_url: route_url.clone(),
                image_urls: image_urls.clone(),
                token: token.clone(),
                initial_index: idx,
                route_fn,
                on_close: move |_| carousel_idx.set(None),
            }
        }
    }
}

#[component]
fn FeedCollage(
    route_url: Option<String>,
    image_urls: Vec<String>,
    token: Option<String>,
    map_height: String,
    on_open: EventHandler<usize>,
    route_fn: RouteFn,
) -> Element {
    let has_map = route_url.is_some();
    let img_count = image_urls.len();

    let max_img_tiles: usize = if has_map { 2 } else { 4 };
    let show_imgs = img_count.min(max_img_tiles);
    let overflow = img_count.saturating_sub(max_img_tiles);

    let grid_class = if has_map {
        match show_imgs {
            0 => format!(
                "{} media-collage {} media-collage-map-only",
                Styles::media_collage,
                Styles::media_collage_map_only
            ),
            1 => format!(
                "{} media-collage {} media-collage-map-1",
                Styles::media_collage,
                Styles::media_collage_map_1
            ),
            _ => format!(
                "{} media-collage {} media-collage-map-2",
                Styles::media_collage,
                Styles::media_collage_map_2
            ),
        }
    } else {
        match show_imgs {
            1 => format!(
                "{} media-collage {} media-collage-1",
                Styles::media_collage,
                Styles::media_collage_1
            ),
            2 => format!(
                "{} media-collage {} media-collage-2",
                Styles::media_collage,
                Styles::media_collage_2
            ),
            3 => format!(
                "{} media-collage {} media-collage-3",
                Styles::media_collage,
                Styles::media_collage_3
            ),
            _ => format!(
                "{} media-collage {} media-collage-4",
                Styles::media_collage,
                Styles::media_collage_4
            ),
        }
    };

    rsx! {
        div { class: "{grid_class}",
            if let Some(ref url) = route_url {
                div {
                    class: format!("{} collage-cell {} collage-map-tile", Styles::collage_cell, Styles::collage_map_tile),
                    onclick: move |_| on_open.call(0),
                    RouteSection {
                        route_url: url.clone(),
                        token: token.clone(),
                        map_height: map_height.clone(),
                        interactive: false,
                        route_fn,
                    }
                    div { class: Styles::collage_map_hover_hint }
                }
            }
            {(0..show_imgs).map(|i| {
                let url = image_urls[i].clone();
                let slide_idx = if has_map { i + 1 } else { i };
                let is_last = i + 1 == show_imgs && overflow > 0;
                rsx! {
                    div {
                        key: "{i}",
                        class: "{Styles::collage_cell} collage-cell",
                        onclick: move |_| on_open.call(slide_idx),
                        img { class: "{Styles::collage_img} collage-img", src: "{url}", alt: "", loading: "lazy" }
                        if is_last {
                            div { class: "{Styles::collage_overflow_badge} collage-overflow-badge", "+{overflow}" }
                        }
                    }
                }
            })}
        }
    }
}

#[component]
fn DetailLayout(
    route_url: Option<String>,
    image_urls: Vec<String>,
    token: Option<String>,
    map_height: String,
    route_fn: RouteFn,
) -> Element {
    let img_count = image_urls.len();
    let mut lightbox_idx: Signal<Option<usize>> = use_signal(|| None);

    rsx! {
        div { class: "{Styles::media_collage_detail} media-collage-detail",
            if let Some(ref url) = route_url {
                RouteSection {
                    route_url: url.clone(),
                    token: token.clone(),
                    map_height: map_height.clone(),
                    interactive: true,
                    show_elevation: true,
                    route_fn,
                }
            }
            if img_count > 0 {
                ImageGrid { urls: image_urls.clone(), on_click: move |i| lightbox_idx.set(Some(i)) }
            }
            if let Some(idx) = *lightbox_idx.read() {
                ImageLightbox {
                    urls: image_urls.clone(),
                    initial_index: idx,
                    on_close: move |_| lightbox_idx.set(None),
                }
            }
        }
    }
}

#[component]
fn ImageGrid(urls: Vec<String>, on_click: EventHandler<usize>) -> Element {
    let count = urls.len();
    let show = count.min(4);
    let overflow = count.saturating_sub(4);

    let grid_class = match show {
        1 => format!(
            "{} media-collage {} media-collage-1",
            Styles::media_collage,
            Styles::media_collage_1
        ),
        2 => format!(
            "{} media-collage {} media-collage-2",
            Styles::media_collage,
            Styles::media_collage_2
        ),
        3 => format!(
            "{} media-collage {} media-collage-3",
            Styles::media_collage,
            Styles::media_collage_3
        ),
        _ => format!(
            "{} media-collage {} media-collage-4",
            Styles::media_collage,
            Styles::media_collage_4
        ),
    };

    rsx! {
        div { class: "{grid_class}",
            {(0..show).map(|i| {
                let url = urls[i].clone();
                let is_last = i + 1 == show && overflow > 0;
                rsx! {
                    div {
                        key: "{i}",
                        class: "{Styles::collage_cell} collage-cell",
                        onclick: move |_| on_click.call(i),
                        img { class: "{Styles::collage_img} collage-img", src: "{url}", alt: "", loading: "lazy" }
                        if is_last {
                            div { class: "{Styles::collage_overflow_badge} collage-overflow-badge", "+{overflow}" }
                        }
                    }
                }
            })}
        }
    }
}

#[component]
pub fn CarouselOverlay(
    route_url: Option<String>,
    image_urls: Vec<String>,
    token: Option<String>,
    initial_index: usize,
    on_close: EventHandler<()>,
    route_fn: RouteFn,
) -> Element {
    let has_map = route_url.is_some();
    let slide_count = if has_map { 1 } else { 0 } + image_urls.len();
    let mut current = use_signal(|| initial_index);
    let idx = *current.read();
    let is_map_active = has_map && idx == 0;
    let mut swipe_start: Signal<Option<f64>> = use_signal(|| None);

    let overlay_class = if is_map_active {
        format!(
            "{} carousel-overlay {} carousel-map-active",
            Styles::carousel_overlay,
            Styles::carousel_map_active
        )
    } else {
        format!("{} carousel-overlay", Styles::carousel_overlay)
    };

    rsx! {
        div {
            class: "{overlay_class}",
            tabindex: "-1",
            autofocus: true,
            onclick: move |_| on_close.call(()),
            onkeydown: move |e| {
                match e.key() {
                    Key::Escape => on_close.call(()),
                    Key::ArrowLeft => { let i = *current.read(); if i > 0 { current.set(i - 1); } }
                    Key::ArrowRight => { let i = *current.read(); if i + 1 < slide_count { current.set(i + 1); } }
                    _ => {}
                }
            },
            onpointerdown: move |e| { swipe_start.set(Some(e.client_coordinates().x)); },
            onpointerup: move |e| {
                let start = *swipe_start.read();
                swipe_start.set(None);
                if let Some(sx) = start {
                    let delta = e.client_coordinates().x - sx;
                    let i = *current.read();
                    if delta > 50.0 && i > 0 { current.set(i - 1); }
                    else if delta < -50.0 && i + 1 < slide_count { current.set(i + 1); }
                }
            },
            button {
                class: Styles::carousel_close_btn,
                onclick: move |_| on_close.call(()),
                i { class: "ph ph-x" }
            }
            div {
                class: Styles::carousel_overlay_content,
                onclick: move |e| e.stop_propagation(),
                if let Some(ref url) = route_url {
                    div {
                        class: if idx == 0 {
                            format!("{} {}", Styles::carousel_overlay_slide, Styles::carousel_overlay_slide_active)
                        } else {
                            format!("{} {}", Styles::carousel_overlay_slide, Styles::carousel_overlay_slide_hidden)
                        },
                        RouteSection {
                            route_url: url.clone(),
                            token: token.clone(),
                            map_height: "100vh".to_string(),
                            interactive: true,
                            route_fn,
                        }
                    }
                }
                {image_urls.iter().enumerate().map(|(i, url)| {
                    let slide_idx = if has_map { i + 1 } else { i };
                    let url = url.clone();
                    rsx! {
                        div {
                            key: "{i}",
                            class: if idx == slide_idx {
                                format!("{} {}", Styles::carousel_overlay_slide, Styles::carousel_overlay_slide_active)
                            } else {
                                format!("{} {}", Styles::carousel_overlay_slide, Styles::carousel_overlay_slide_hidden)
                            },
                            img { class: Styles::carousel_overlay_img, src: "{url}", alt: "" }
                        }
                    }
                })}
                if slide_count > 1 {
                    if idx > 0 {
                        button {
                            class: format!("{} {}", Styles::carousel_nav, Styles::carousel_nav_prev),
                            onclick: move |e| { e.stop_propagation(); let i = *current.read(); if i > 0 { current.set(i - 1); } },
                            i { class: "ph ph-caret-left" }
                        }
                    }
                    if idx + 1 < slide_count {
                        button {
                            class: format!("{} {}", Styles::carousel_nav, Styles::carousel_nav_next),
                            onclick: move |e| { e.stop_propagation(); let i = *current.read(); if i + 1 < slide_count { current.set(i + 1); } },
                            i { class: "ph ph-caret-right" }
                        }
                    }
                    div { class: Styles::carousel_indicator,
                        {(0..slide_count).map(|i| rsx! {
                            button {
                                key: "{i}",
                                class: if i == idx {
                                    format!("{} {}", Styles::carousel_dot, Styles::carousel_dot_active)
                                } else {
                                    Styles::carousel_dot.to_string()
                                },
                                onclick: move |_| current.set(i),
                                aria_label: "Slide {i + 1}",
                            }
                        })}
                        span { class: Styles::carousel_counter, "{idx + 1} / {slide_count}" }
                    }
                }
            }
        }
    }
}

#[component]
fn ImageLightbox(urls: Vec<String>, initial_index: usize, on_close: EventHandler<()>) -> Element {
    let mut current = use_signal(|| initial_index);
    let count = urls.len();
    let idx = *current.read();
    let url = urls.get(idx).cloned().unwrap_or_default();
    let mut swipe_start: Signal<Option<f64>> = use_signal(|| None);

    rsx! {
        div {
            class: Styles::image_lightbox,
            tabindex: "-1",
            autofocus: true,
            onclick: move |_| on_close.call(()),
            onkeydown: move |e| {
                match e.key() {
                    Key::Escape => on_close.call(()),
                    Key::ArrowLeft => { let i = *current.read(); if i > 0 { current.set(i - 1); } }
                    Key::ArrowRight => { let i = *current.read(); if i + 1 < count { current.set(i + 1); } }
                    _ => {}
                }
            },
            onpointerdown: move |e| { swipe_start.set(Some(e.client_coordinates().x)); },
            onpointerup: move |e| {
                let start = *swipe_start.read();
                swipe_start.set(None);
                if let Some(sx) = start {
                    let delta = e.client_coordinates().x - sx;
                    let i = *current.read();
                    if delta > 50.0 && i > 0 { current.set(i - 1); }
                    else if delta < -50.0 && i + 1 < count { current.set(i + 1); }
                }
            },
            button {
                class: Styles::carousel_close_btn,
                onclick: move |_| on_close.call(()),
                i { class: "ph ph-x" }
            }
            div {
                class: Styles::lightbox_content,
                onclick: move |e| e.stop_propagation(),
                img { class: Styles::lightbox_img, src: "{url}", alt: "" }
                if count > 1 {
                    if idx > 0 {
                        button {
                            class: format!("{} {}", Styles::carousel_nav, Styles::carousel_nav_prev),
                            onclick: move |_| { let i = *current.read(); if i > 0 { current.set(i - 1); } },
                            i { class: "ph ph-caret-left" }
                        }
                    }
                    if idx + 1 < count {
                        button {
                            class: format!("{} {}", Styles::carousel_nav, Styles::carousel_nav_next),
                            onclick: move |_| { let i = *current.read(); if i + 1 < count { current.set(i + 1); } },
                            i { class: "ph ph-caret-right" }
                        }
                    }
                    div { class: Styles::carousel_indicator,
                        {(0..count).map(|i| rsx! {
                            button {
                                key: "{i}",
                                class: if i == idx {
                                    format!("{} {}", Styles::carousel_dot, Styles::carousel_dot_active)
                                } else {
                                    Styles::carousel_dot.to_string()
                                },
                                onclick: move |_| current.set(i),
                                aria_label: "Slide {i + 1}",
                            }
                        })}
                        span { class: Styles::carousel_counter, "{idx + 1} / {count}" }
                    }
                }
            }
        }
    }
}

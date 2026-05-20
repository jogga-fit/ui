use std::{future::Future, pin::Pin};

pub mod browser;
pub mod components;
pub mod exercise;
pub mod fns;
pub mod format;
pub mod hooks;
pub mod image;
pub mod pages;
pub mod shell;
pub mod types;

pub use fns::*;
pub use hooks::use_client_only;

pub type FutureResult<F> = Pin<Box<dyn Future<Output = Result<F, String>>>>;
pub type RoutePoint = (f64, f64, Option<f64>);

#[dioxus::prelude::component]
pub fn UiStyles() -> dioxus::prelude::Element {
    use dioxus::prelude::*;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/components/feed_gate/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/person_row/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/tab_bar/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/actor_link/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/avatar/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/badge/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/crop_modal/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/dialog/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/empty_state/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/error_banner/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/exercise_stats_grid/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/like_button/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/media_carousel/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/pages/auth/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/pages/clubs/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/pages/credits/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/pages/home/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/pages/people/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/pages/profile/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/pages/settings/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/post/feed_card.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/post/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/post_menu/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/remote_follow_card/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/route_map/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/setting_toggle/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/skeleton/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/stat_toggle_chips/style.css", AssetOptions::css_module()) }
        document::Link { rel: "stylesheet", href: asset!("/src/components/switch/style.css", AssetOptions::css_module()) }
    }
}

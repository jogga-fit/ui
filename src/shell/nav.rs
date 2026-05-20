use super::Styles;
use super::types::PageNav;
use dioxus::prelude::*;

#[css_module("/src/shell/nav.css")]
pub struct NavStyles;

pub fn class_with_active(
    base: impl std::fmt::Display,
    active: impl std::fmt::Display,
    is_active: bool,
) -> String {
    if is_active {
        format!("{base} {active}")
    } else {
        base.to_string()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NavLinkVariant {
    Sidebar,
    Bottom,
}

#[component]
pub fn ShellNavLink(item: PageNav, active: PageNav, variant: NavLinkVariant) -> Element {
    let is_active = item.key == active.key;
    let class = match variant {
        NavLinkVariant::Sidebar => {
            class_with_active(Styles::nav_item, Styles::nav_item_active, is_active)
        }
        NavLinkVariant::Bottom => class_with_active(
            Styles::bottom_nav_item,
            Styles::bottom_nav_item_active,
            is_active,
        ),
    };

    let testid = match variant {
        NavLinkVariant::Bottom => Some("bottom-nav-item"),
        NavLinkVariant::Sidebar => None,
    };
    rsx! {
        Link {
            class,
            to: item.href,
            "data-testid": testid,
            i { class: format!("{} {}", item.icon_class, Styles::nav_icon) }
            span { "{item.label}" }
        }
    }
}

#[component]
pub fn Brand(brand: String, brand_icon: String, compact: bool) -> Element {
    let class = if compact {
        Styles::mobile_header_brand
    } else {
        Styles::sidebar_brand
    };
    let icon_class = if compact {
        brand_icon
    } else {
        format!("{brand_icon} {}", Styles::brand_icon)
    };

    rsx! {
        div { class,
            i { class: icon_class }
            span { class: Styles::brand_name, "{brand}" }
        }
    }
}

#[component]
pub fn AccountSheetLink(item: PageNav, active: PageNav, on_follow: EventHandler<()>) -> Element {
    let class = class_with_active(
        NavStyles::account_sheet_item,
        NavStyles::account_sheet_item_active,
        item == active,
    );

    rsx! {
        Link {
            class,
            to: item.href,
            onclick: move |_| on_follow.call(()),
            i { class: item.icon_class }
            span { "{item.label}" }
            i { class: format!("ph ph-caret-right {}", NavStyles::sheet_arrow) }
        }
        div { class: NavStyles::account_sheet_divider }
    }
}

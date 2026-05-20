pub mod nav;
pub mod types;

use dioxus::prelude::*;

#[css_module("/src/shell/style.css")]
pub struct Styles;

use nav::{AccountSheetLink, Brand, NavLinkVariant, NavStyles, ShellNavLink, class_with_active};
pub use types::{AppShellNavItem, AppShellNavPlacement, AppShellUser};

use crate::components::notifications::{
    NotificationItem, NotificationList, Styles as NotificationStyles,
};

#[component]
pub fn AppShell(
    active: String,
    nav_items: Vec<AppShellNavItem>,
    #[props(default = "Jogga".to_string())] brand: String,
    #[props(default = "ph ph-person-simple-run".to_string())] brand_icon: String,
    #[props(default)] user: Option<AppShellUser>,
    #[props(default)] notifications: Option<Vec<NotificationItem>>,
    #[props(default)] on_signin: Option<EventHandler<()>>,
    #[props(default)] on_signout: Option<EventHandler<()>>,
    #[props(default)] on_notification_dismiss: Option<EventHandler<String>>,
    children: Element,
) -> Element {
    let mut account_open = use_signal(|| false);
    let mut notifications_open = use_signal(|| false);
    let dismissed_notification_ids: Signal<Vec<String>> = use_signal(Vec::new);

    let is_logged_in = user.is_some();
    let display_initial = user
        .as_ref()
        .map(AppShellUser::initial)
        .unwrap_or_else(|| "?".to_string());
    let account_name = user
        .as_ref()
        .map(|u| u.name().to_string())
        .unwrap_or_default();
    let account_handle = user
        .as_ref()
        .map(|u| format!("@{}", u.username))
        .unwrap_or_default();

    let show_notifications = is_logged_in && notifications.is_some();

    // Recomputes only when dismissed_notification_ids changes, not on every
    // account_open / notifications_open toggle.
    let visible_notifications = use_memo(move || {
        let hidden_ids = dismissed_notification_ids.read();
        notifications
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter(|n| !hidden_ids.iter().any(|id| id == &n.id))
            .cloned()
            .collect::<Vec<_>>()
    });
    let unread_count = use_memo(move || {
        visible_notifications
            .read()
            .iter()
            .filter(|n| !n.is_read)
            .count()
    });

    let sidebar_nav_items = nav_items
        .iter()
        .filter(|item| {
            matches!(
                item.placement,
                AppShellNavPlacement::Primary | AppShellNavPlacement::DesktopOnly
            )
        })
        .cloned()
        .collect::<Vec<_>>();
    let sidebar_account_items = nav_items
        .iter()
        .filter(|item| item.placement == AppShellNavPlacement::Account)
        .cloned()
        .collect::<Vec<_>>();
    let bottom_nav_items = nav_items
        .iter()
        .filter(|item| item.placement == AppShellNavPlacement::Primary)
        .cloned()
        .collect::<Vec<_>>();
    let sheet_account_items = nav_items
        .iter()
        .filter(|item| {
            matches!(
                item.placement,
                AppShellNavPlacement::DesktopOnly | AppShellNavPlacement::Account
            )
        })
        .cloned()
        .collect::<Vec<_>>();

    rsx! {
        div { class: Styles::app_shell,
            nav { class: Styles::sidebar, "data-testid": "sidebar",
                Brand { brand: brand.clone(), brand_icon: brand_icon.clone(), compact: false }

                for item in sidebar_nav_items {
                    ShellNavLink {
                        key: "{item.key}",
                        item,
                        active: active.clone(),
                        variant: NavLinkVariant::Sidebar,
                    }
                }

                div { class: Styles::sidebar_spacer }

                if show_notifications {
                    div { class: NotificationStyles::notification_section,
                        button {
                            class: class_with_active(
                                Styles::nav_item,
                                Styles::nav_item_active,
                                *notifications_open.read(),
                            ),
                            onclick: move |_| {
                                let next = !*notifications_open.read();
                                notifications_open.set(next);
                                if next {
                                    account_open.set(false);
                                }
                            },
                            i { class: format!("ph ph-bell {}", Styles::nav_icon) }
                            span { "Notifications" }
                            if *unread_count.read() > 0 {
                                span { class: NotificationStyles::notification_badge, "{unread_count}" }
                            }
                        }
                        if *notifications_open.read() {
                            div { class: NotificationStyles::notification_inline_list,
                                div { class: NotificationStyles::notification_panel_header,
                                    span { class: NotificationStyles::notification_panel_label, "Notifications" }
                                }
                                NotificationList {
                                    notifications: visible_notifications.read().clone(),
                                    dismissed_ids: dismissed_notification_ids,
                                    on_dismiss: on_notification_dismiss,
                                }
                            }
                        }
                    }
                }

                if is_logged_in {
                    for item in sidebar_account_items {
                        ShellNavLink {
                            key: "{item.key}",
                            item,
                            active: active.clone(),
                            variant: NavLinkVariant::Sidebar,
                        }
                    }
                    if let Some(signout) = on_signout {
                        button {
                            class: format!("{} {}", Styles::nav_item, Styles::sign_out_btn),
                            "data-testid": "sign-out-btn",
                            onclick: move |_| signout.call(()),
                            i { class: format!("ph ph-sign-out {}", Styles::nav_icon) }
                            span { "Sign out" }
                        }
                    }
                } else if let Some(signin) = on_signin {
                    button {
                        class: "{Styles::sign_in_btn} {Styles::nav_item}",
                        "data-testid": "sidebar-signin",
                        onclick: move |_| signin.call(()),
                        i { class: "ph ph-sign-in" }
                        span { "Sign in" }
                    }
                }
            }

            header { class: Styles::mobile_header,
                Brand { brand: brand.clone(), brand_icon: brand_icon.clone(), compact: true }
                div { class: Styles::mobile_header_actions,
                    if is_logged_in {
                        if show_notifications {
                            button {
                                class: Styles::mobile_header_btn,
                                "data-testid": "mobile-header-btn",
                                onclick: move |_| {
                                    let next = !*notifications_open.read();
                                    notifications_open.set(next);
                                    if next {
                                        account_open.set(false);
                                    }
                                },
                                i { class: "ph ph-bell" }
                                if *unread_count.read() > 0 {
                                    span { class: Styles::mobile_header_badge, "{unread_count}" }
                                }
                            }
                        }
                        button {
                            class: class_with_active(
                                Styles::mobile_header_avatar,
                                Styles::mobile_header_avatar_open,
                                *account_open.read(),
                            ),
                            "data-testid": "mobile-header-avatar",
                            onclick: move |_| {
                                let next = !*account_open.read();
                                account_open.set(next);
                                if next {
                                    notifications_open.set(false);
                                }
                            },
                            "{display_initial}"
                        }
                    } else if let Some(signin) = on_signin {
                        button {
                            class: Styles::mobile_sign_in_btn,
                            "data-testid": "mobile-header-signin",
                            onclick: move |_| signin.call(()),
                            i { class: "ph ph-sign-in" }
                            span { "Sign in" }
                        }
                    }
                }
            }

            main { class: Styles::main_content,
                {children}
            }

            if !bottom_nav_items.is_empty() {
                nav { class: Styles::bottom_nav, "data-testid": "bottom-nav",
                    for item in bottom_nav_items {
                        ShellNavLink {
                            key: "{item.key}",
                            item,
                            active: active.clone(),
                            variant: NavLinkVariant::Bottom,
                        }
                    }
                }
            }

            if is_logged_in && *account_open.read() {
                div { class: NavStyles::sheet_backdrop, onclick: move |_| account_open.set(false) }
                div { class: NavStyles::sheet,
                    div { class: NavStyles::sheet_handle }
                    div { class: NavStyles::account_sheet_user,
                        div { class: NavStyles::account_sheet_avatar, "{display_initial}" }
                        div {
                            div { class: NavStyles::account_sheet_name, "{account_name}" }
                            div { class: NavStyles::account_sheet_handle, "{account_handle}" }
                        }
                    }
                    div { class: NavStyles::account_sheet_nav,
                        for item in sheet_account_items {
                            AccountSheetLink {
                                key: "{item.key}",
                                item,
                                active: active.clone(),
                                on_follow: move |_| account_open.set(false),
                            }
                        }
                        if on_signout.is_some() {
                            div { class: NavStyles::account_sheet_divider }
                            button {
                                class: format!(
                                    "{} {}",
                                    NavStyles::account_sheet_item,
                                    NavStyles::account_sheet_signout,
                                ),
                                onclick: move |_| {
                                    account_open.set(false);
                                    if let Some(signout) = on_signout {
                                        signout.call(());
                                    }
                                },
                                i { class: "ph ph-sign-out" }
                                span { "Sign out" }
                            }
                        }
                    }
                }
            }

            if show_notifications && *notifications_open.read() {
                div { class: NavStyles::sheet_backdrop, onclick: move |_| notifications_open.set(false) }
                div { class: NavStyles::sheet,
                    div { class: NavStyles::sheet_handle }
                    div { class: NotificationStyles::notification_header,
                        span { class: NotificationStyles::notification_title, "Notifications" }
                        button {
                            class: NotificationStyles::notification_close,
                            "aria-label": "Close notifications",
                            onclick: move |_| notifications_open.set(false),
                            i { class: "ph ph-x" }
                        }
                    }
                    div { class: NotificationStyles::notification_sheet_body,
                        NotificationList {
                            notifications: visible_notifications.read().clone(),
                            dismissed_ids: dismissed_notification_ids,
                            on_dismiss: on_notification_dismiss,
                        }
                    }
                }
            }
        }
    }
}

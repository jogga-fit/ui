#![allow(non_snake_case)]

use dioxus::prelude::*;
use jogga_ui::types::{AuthSignal, AuthUser};

#[derive(Clone, PartialEq, Debug)]
pub enum ActivePage {
    Feed,
    Profile,
    Settings,
    Post,
    People,
    Clubs,
}

#[component]
pub fn AppShell(active: ActivePage, children: Element) -> Element {
    let mut acct_open: Signal<bool> = use_signal(|| false);
    let auth = use_context::<AuthSignal>();

    let is_logged_in = auth.read().is_some();

    let nav_class = |page: &ActivePage| {
        if *page == active {
            "nav-item nav-item-active"
        } else {
            "nav-item"
        }
    };
    let bottom_class = |page: &ActivePage| {
        if *page == active {
            "bottom-nav-item bottom-nav-item-active"
        } else {
            "bottom-nav-item"
        }
    };

    let do_signout = use_callback(move |_: ()| {
        spawn(async move {
            let _ =
                document::eval("document.cookie='jogga_auth=; path=/; max-age=0; SameSite=Lax';")
                    .await;
        });
        auth.clone().set(None);
    });

    let do_signin = use_callback(move |_: ()| {
        spawn(async move {
            let _ = document::eval(
                "document.cookie='jogga_auth=alex:demo-token; path=/; max-age=2592000; SameSite=Lax';"
            ).await;
        });
        auth.clone().set(Some(AuthUser {
            token: "demo-token".to_string(),
            username: "alex".to_string(),
            ap_id: "https://jogga.fit/users/alex".to_string(),
        }));
    });

    let display_initial = auth
        .read()
        .as_ref()
        .and_then(|u| u.username.chars().next())
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/app_shell.css") }
        div { class: "app-shell",
            // Sidebar (desktop)
            nav { class: "sidebar",
                div { class: "sidebar-brand",
                    i { class: "ph ph-person-simple-run brand-icon" }
                    span { class: "brand-name", "Jogga" }
                }

                Link { class: nav_class(&ActivePage::Feed), to: "/feed",
                    i { class: "ph ph-house nav-icon" }
                    span { "Feed" }
                }
                Link { class: nav_class(&ActivePage::People), to: "/people",
                    i { class: "ph ph-users nav-icon" }
                    span { "People" }
                }
                Link { class: nav_class(&ActivePage::Clubs), to: "/clubs",
                    i { class: "ph ph-users-three nav-icon" }
                    span { "Clubs" }
                }
                if is_logged_in {
                    Link { class: nav_class(&ActivePage::Profile), to: "/profile",
                        i { class: "ph ph-user nav-icon" }
                        span { "Profile" }
                    }
                }

                div { class: "sidebar-spacer" }

                if is_logged_in {
                    Link { class: nav_class(&ActivePage::Settings), to: "/settings",
                        i { class: "ph ph-gear nav-icon" }
                        span { "Settings" }
                    }
                    button {
                        class: "nav-item sign-out-btn",
                        onclick: move |_| do_signout.call(()),
                        i { class: "ph ph-sign-out nav-icon" }
                        span { "Sign out" }
                    }
                } else {
                    button {
                        class: "nav-item sign-in-btn",
                        onclick: move |_| do_signin.call(()),
                        i { class: "ph ph-sign-in nav-icon" }
                        span { "Sign in" }
                    }
                }
            }

            // Mobile header
            header { class: "mobile-header",
                div { class: "mobile-header-brand",
                    i { class: "ph ph-person-simple-run" }
                    span { "Jogga" }
                }
                div { class: "mobile-header-actions",
                    if is_logged_in {
                        button {
                            class: if *acct_open.read() { "mobile-header-avatar mobile-header-avatar-open" } else { "mobile-header-avatar" },
                            onclick: move |_| { let v = !*acct_open.read(); acct_open.set(v); },
                            "{display_initial}"
                        }
                    } else {
                        button { class: "mobile-header-signin", onclick: move |_| do_signin.call(()),
                            i { class: "ph ph-sign-in" }
                            "Sign in"
                        }
                    }
                }
            }

            // Main content area
            main { class: "main-content",
                {children}
            }

            // Mobile bottom nav
            nav { class: "bottom-nav",
                Link { class: bottom_class(&ActivePage::Feed), to: "/feed",
                    i { class: "ph ph-house nav-icon" }
                    span { "Feed" }
                }
                Link { class: bottom_class(&ActivePage::People), to: "/people",
                    i { class: "ph ph-users nav-icon" }
                    span { "People" }
                }
                Link { class: bottom_class(&ActivePage::Clubs), to: "/clubs",
                    i { class: "ph ph-users-three nav-icon" }
                    span { "Clubs" }
                }
            }

            // Account sheet (mobile)
            if *acct_open.read() {
                div { class: "notif-sheet-backdrop", onclick: move |_| acct_open.set(false) }
                div { class: "notif-sheet",
                    div { class: "notif-sheet-handle" }
                    div { class: "acct-sheet-user",
                        div { class: "acct-sheet-avatar", "{display_initial}" }
                        div {
                            div { class: "acct-sheet-name",
                                "@{auth.read().as_ref().map(|u| u.username.as_str()).unwrap_or(\"?\")}"
                            }
                        }
                    }
                    div { class: "acct-sheet-nav",
                        Link {
                            class: "acct-sheet-item",
                            to: "/profile",
                            onclick: move |_| acct_open.set(false),
                            i { class: "ph ph-user" }
                            span { "Profile" }
                            i { class: "ph ph-caret-right acct-sheet-arrow" }
                        }
                        div { class: "acct-sheet-divider" }
                        Link {
                            class: "acct-sheet-item",
                            to: "/settings",
                            onclick: move |_| acct_open.set(false),
                            i { class: "ph ph-gear" }
                            span { "Settings" }
                            i { class: "ph ph-caret-right acct-sheet-arrow" }
                        }
                        div { class: "acct-sheet-divider" }
                        button {
                            class: "acct-sheet-item acct-sheet-signout",
                            onclick: move |_| {
                                acct_open.set(false);
                                do_signout.call(());
                            },
                            i { class: "ph ph-sign-out" }
                            span { "Sign out" }
                        }
                    }
                }
            }
        }
    }
}

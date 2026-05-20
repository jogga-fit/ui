#![allow(non_snake_case, unused_braces)]

mod mock;
mod pages;

use dioxus::prelude::*;
use jogga_ui::{
    UiStyles, browser,
    components::{
        actor_link::ActorLink,
        auth_card::AuthCard,
        avatar::{Avatar, AvatarSize},
        badge::{Badge, BadgeVariant, VerifiedIcon},
        empty_state::EmptyState,
        error_banner::ErrorBanner,
        exercise_stats_grid::ExerciseStatsGrid,
        like_button::LikeButton,
        media_carousel::{CarouselOverlay, MediaCollage},
        post::{EditPostModal, ExerciseCard, FeedCard, ReplyComposer, ReplyItem},
        post_menu::PostMenu,
        route_map::RouteMap,
        setting_toggle::SettingToggle,
        skeleton::Skeleton,
        stat_toggle_chips::{STAT_TOGGLES, StatToggleChips},
        switch::{Switch, SwitchThumb},
    },
    pages::MigrationModal,
    shell::{AppShell, types::AppShellUser, types::PageNav},
    types::{AuthSignal, AuthUser, MigrationModalSignal, Theme, ThemeSignal},
};

use mock::*;
use pages::{
    clubs::{ClubDetailPage, ClubsDemoPage},
    feed::FeedPage,
    login::LoginPage,
    people::PeopleDemoPage,
    post::PostPage,
    profile::ProfilePage,
    register::RegisterDemoPage,
    reset_password::ResetPasswordDemoPage,
    settings::SettingsPage,
};

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(AuthLayout)]
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/reset-password")]
    ResetPassword {},
    #[end_layout]
    #[layout(ShellLayout)]
    #[route("/feed")]
    Feed {},
    #[route("/profile")]
    Profile {},
    #[route("/settings")]
    Settings {},
    #[route("/post")]
    Post {},
    #[route("/people")]
    People {},
    #[route("/clubs")]
    Clubs {},
    #[route("/clubs/:handle")]
    ClubDetail { handle: String },
    #[end_layout]
    #[route("/:..segments")]
    Components { segments: Vec<String> },
}

#[component]
fn App() -> Element {
    // Auth: start as None, read from cookie on mount
    let mut auth: AuthSignal = use_signal(|| None);
    use_context_provider(|| auth);

    use_effect(move || {
        let _ = browser::mark_document_hydrated();
    });

    use_effect(move || {
        spawn(async move {
            let js = r#"(function(){
                var p = document.cookie.split('; ').find(function(r){ return r.startsWith('jogga_auth='); });
                return p ? p.slice('jogga_auth='.length) : '';
            })()"#;
            if let Ok(val) = document::eval(js).await
                && let Some(s) = val.as_str()
                && let Some((username, token)) = s.split_once(':')
                && !token.is_empty()
            {
                auth.set(Some(AuthUser {
                    token: token.to_string(),
                    username: username.to_string(),
                    ap_id: format!("https://jogga.fit/users/{username}"),
                }));
            }
        });
    });

    // Theme: read data-theme synchronously — our index.html init script sets it
    // before WASM runs, so this gives the correct saved value immediately.
    // Avoids race where the DOM-apply effect fires first with default "dark"
    // and overwrites the saved cookie before the async cookie-read can run.
    let theme: ThemeSignal = use_signal(|| {
        #[cfg(target_arch = "wasm32")]
        {
            js_sys::eval("document.documentElement.getAttribute('data-theme')")
                .ok()
                .and_then(|v| v.as_string())
                .and_then(|s| match s.as_str() {
                    "dark" => Some(Theme::Dark),
                    "light" => Some(Theme::Light),
                    "system" => Some(Theme::System),
                    _ => None,
                })
                .unwrap_or_default()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Theme::default()
        }
    });
    use_context_provider(|| theme);

    let migration_modal: MigrationModalSignal = use_signal(|| None);
    use_context_provider(|| migration_modal);

    use_effect(move || {
        let pref = *theme.peek();
        spawn(async move {
            // Write theme cookie
            let cookie_js = format!(
                "document.cookie='jogga_theme={pref}; path=/; max-age=2592000; SameSite=Lax';"
            );
            let _ = document::eval(&cookie_js).await;
            // Apply to DOM
            if pref == Theme::System {
                let js = r#"(function() {
                    var mq = window.matchMedia('(prefers-color-scheme: dark)');
                    document.documentElement.setAttribute('data-theme', mq.matches ? 'dark' : 'light');
                    if (window._joggaThemeHandler) { mq.removeEventListener('change', window._joggaThemeHandler); }
                    window._joggaThemeHandler = function(e) { document.documentElement.setAttribute('data-theme', e.matches ? 'dark' : 'light'); };
                    mq.addEventListener('change', window._joggaThemeHandler);
                    window._joggaThemeMQ = mq;
                })();"#;
                let _ = document::eval(js).await;
            } else {
                let js = format!(
                    r#"(function() {{
                    if (window._joggaThemeMQ && window._joggaThemeHandler) {{
                        window._joggaThemeMQ.removeEventListener('change', window._joggaThemeHandler);
                        window._joggaThemeMQ = null; window._joggaThemeHandler = null;
                    }}
                    document.documentElement.setAttribute('data-theme', '{pref}');
                }})();"#
                );
                let _ = document::eval(&js).await;
            }
        });
    });

    rsx! {
        UiStyles {}
        document::Link { rel: "stylesheet", href: asset!("/assets/demo.css") }
        DemoPageNav {}
        Router::<Route> {}
        if let Some(profile) = migration_modal.read().clone() {
            MigrationModal {
                profile,
                on_close: move |_| migration_modal.clone().set(None),
                add_alias_fn: Callback::new(mock_add_alias),
                remove_alias_fn: Callback::new(mock_remove_alias),
                move_account_fn: Callback::new(mock_move_account),
            }
        }
    }
}

#[component]
fn AuthLayout() -> Element {
    let route = use_route::<Route>();
    let subtitle = match route {
        Route::Login {} => "Sign in to your account",
        Route::Register {} => "Create your account",
        Route::ResetPassword {} => "Verify your identity",
        _ => "",
    };
    rsx! {
        AuthCard { subtitle,
            Outlet::<Route> {}
        }
    }
}

#[component]
fn ShellLayout() -> Element {
    let route = use_route::<Route>();
    let active = match route {
        Route::Feed {} => NAV_FEED,
        Route::Profile {} => NAV_PROFILE,
        Route::Settings {} => NAV_SETTINGS,
        Route::Post {} => NAV_POST,
        Route::People {} => NAV_PEOPLE,
        Route::Clubs {} | Route::ClubDetail { .. } => NAV_CLUBS,
        _ => NAV_FEED,
    };
    let auth = use_context::<AuthSignal>();
    let current_user = auth.read().clone();
    let shell_user = current_user
        .as_ref()
        .map(|user| AppShellUser::new(user.username.clone()));
    let nav_items = shell_nav_items(&current_user);

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

    rsx! {
        AppShell {
            active,
            nav_items,
            user: shell_user,
            on_signin: move |_| do_signin.call(()),
            on_signout: move |_| do_signout.call(()),
            Outlet::<Route> {}
        }
    }
}

const NAV_FEED: PageNav = PageNav::primary("feed", "Feed", "/feed", "ph ph-house");
const NAV_PEOPLE: PageNav = PageNav::primary("people", "People", "/people", "ph ph-users");
const NAV_CLUBS: PageNav = PageNav::primary("clubs", "Clubs", "/clubs", "ph ph-users-three");
const NAV_PROFILE: PageNav = PageNav::user("profile", "Profile", "/profile", "ph ph-user");
const NAV_SETTINGS: PageNav = PageNav::user("settings", "Settings", "/settings", "ph ph-gear");
const NAV_POST: PageNav = PageNav::primary("post", "Post", "/post", "ph ph-article");

fn shell_nav_items(user: &Option<AuthUser>) -> Vec<PageNav> {
    let mut items = vec![NAV_FEED, NAV_PEOPLE, NAV_CLUBS];
    if user.is_some() {
        items.push(NAV_PROFILE);
        items.push(NAV_SETTINGS);
    }
    items
}

#[component]
fn Login() -> Element {
    rsx! { LoginPage {} }
}
#[component]
fn Register() -> Element {
    rsx! { RegisterDemoPage {} }
}
#[component]
fn ResetPassword() -> Element {
    rsx! { ResetPasswordDemoPage {} }
}
#[component]
fn Feed() -> Element {
    rsx! { FeedPage {} }
}
#[component]
fn Profile() -> Element {
    rsx! { ProfilePage {} }
}
#[component]
fn Settings() -> Element {
    rsx! { SettingsPage {} }
}
#[component]
fn Post() -> Element {
    rsx! { PostPage {} }
}
#[component]
fn People() -> Element {
    rsx! { PeopleDemoPage {} }
}
#[component]
fn Clubs() -> Element {
    rsx! { ClubsDemoPage {} }
}
#[component]
fn ClubDetail(handle: String) -> Element {
    rsx! { ClubDetailPage { handle } }
}

#[component]
#[allow(unused_variables)]
fn Components(segments: Vec<String>) -> Element {
    rsx! { DemoPage {} }
}

#[component]
fn DemoPageNav() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/demo_page.css") }
        nav {
            class: if *open.read() { "demo-page-nav demo-nav-open" } else { "demo-page-nav" },
            // Mobile-only toggle button
            button {
                class: "demo-nav-toggle",
                onclick: move |_| { let v = !*open.read(); open.set(v); },
                i { class: if *open.read() { "ph ph-x" } else { "ph ph-squares-four" } }
                span { if *open.read() { "Close" } else { "Pages" } }
            }
            // Links — inline on desktop, dropdown on mobile
            div { class: "demo-nav-links",
                a { href: "/login",    onclick: move |_| open.set(false), i { class: "ph ph-sign-in" }      " Login" }
                a { href: "/feed",     onclick: move |_| open.set(false), i { class: "ph ph-house" }        " Feed" }
                a { href: "/people",   onclick: move |_| open.set(false), i { class: "ph ph-users" }        " People" }
                a { href: "/clubs",    onclick: move |_| open.set(false), i { class: "ph ph-users-three" }  " Clubs" }
                a { href: "/profile",  onclick: move |_| open.set(false), i { class: "ph ph-user" }         " Profile" }
                a { href: "/settings", onclick: move |_| open.set(false), i { class: "ph ph-gear" }         " Settings" }
                a { href: "/post",     onclick: move |_| open.set(false), i { class: "ph ph-article" }      " Post" }
                div { class: "demo-page-nav-divider" }
                a { href: "/",         onclick: move |_| open.set(false), i { class: "ph ph-puzzle-piece" } " Components" }
            }
        }
    }
}

#[component]
fn DemoPage() -> Element {
    let token = Some("demo-token-abc".to_string());

    let like_fn = Callback::new(mock_like);
    let unlike_fn = Callback::new(mock_like);
    let delete_fn = Callback::new(mock_delete);
    let route_fn = Callback::new(mock_route);
    let update_fn = Callback::new(mock_update_post);
    let reply_fn = Callback::new(mock_create_reply);

    let mut switch_on = use_signal(|| false);
    let mut setting_on = use_signal(|| true);
    let mut show_carousel = use_signal(|| false);
    let mut show_edit_modal = use_signal(|| false);
    let hidden_stats = use_signal(Vec::<String>::new);

    rsx! {
        div { class: "demo-root",
            h1 { class: "demo-title", "Jogga UI · Component Demo" }
            p { class: "demo-subtitle",
                "All reusable components in one scrollable page. Fn-pointer props simulate a 500 ms server delay."
            }

            Section { title: "Avatar",
                div { class: "demo-row demo-row-gap",
                    Avatar { url: None, name: "Sam Cyclist".to_string() }
                    Avatar { url: None, name: "Maya Swimmer".to_string(), size: AvatarSize::Medium }
                    Avatar { url: None, name: "Lara Runner".to_string(), size: AvatarSize::Large }
                    Avatar { url: Some("https://i.pravatar.cc/80?img=5".to_string()), name: "With photo".to_string(), size: AvatarSize::Medium }
                }
            }

            Section { title: "Badge + Verified Icon",
                div { class: "demo-row demo-row-gap",
                    Badge { variant: BadgeVariant::Primary,     "Primary"     }
                    Badge { variant: BadgeVariant::Secondary,   "Secondary"   }
                    Badge { variant: BadgeVariant::Destructive, "Destructive" }
                    Badge { variant: BadgeVariant::Outline,     "Outline"     }
                    span { class: "demo-row", VerifiedIcon {} span { class: "demo-muted", "VerifiedIcon" } }
                }
            }

            Section { title: "Actor Link",
                div { class: "demo-col",
                    ActorLink { is_local: true, ap_id: "https://jogga.fit/users/alex".to_string(), username: "alex".to_string(), display_name: Some("Alex Runner".to_string()), domain: "jogga.fit".to_string(), avatar_url: None, profile_href: "/users/alex".to_string() }
                    ActorLink { is_local: false, ap_id: "https://mastodon.social/users/remotejogger".to_string(), username: "remotejogger".to_string(), display_name: None, domain: "mastodon.social".to_string(), avatar_url: None, profile_href: "https://mastodon.social/users/remotejogger".to_string() }
                    ActorLink { is_local: true, ap_id: "https://jogga.fit/users/sam".to_string(), username: "sam".to_string(), display_name: Some("Sam Cyclist".to_string()), domain: "jogga.fit".to_string(), avatar_url: None, profile_href: "/users/sam".to_string(), club_href: Some("/clubs/sfcycling".to_string()), via_club_display: Some("SF Cycling Club".to_string()) }
                }
            }

            Section { title: "Switch",
                div { class: "demo-row demo-row-gap",
                    Switch { checked: *switch_on.read(), on_checked_change: move |v: bool| switch_on.set(v), SwitchThumb {} }
                    span { class: "demo-muted", "checked = {switch_on}" }
                }
            }

            Section { title: "Setting Toggle",
                div { class: "demo-col",
                    SettingToggle { label: "Email notifications".to_string(), description: "Receive alerts about new followers and likes".to_string(), checked: *setting_on.read(), disabled: false, onchange: move |v| setting_on.set(v) }
                    SettingToggle { label: "Public profile".to_string(), description: "Allow anyone to view your workouts".to_string(), checked: false, disabled: false, onchange: move |_| {} }
                    SettingToggle { label: "Location sharing (disabled)".to_string(), description: "Account plan does not include live tracking".to_string(), checked: false, disabled: true, onchange: move |_| {} }
                }
            }

            Section { title: "Empty State",
                EmptyState { icon: "ph ph-person-simple-run".to_string(), title: "No activities yet".to_string(),
                    span { style: "color: var(--text-muted); font-size: 0.88rem;", "Start recording workouts to see them here." }
                }
            }

            Section { title: "Error Banner",
                ErrorBanner { message: "Failed to load feed — check your connection and try again.".to_string() }
            }

            Section { title: "Skeleton",
                div { class: "demo-col",
                    div { class: "demo-row demo-row-gap",
                        Skeleton { style: "width: 40px; height: 40px; border-radius: 50%;" }
                        div { class: "demo-col",
                            Skeleton { style: "width: 160px; height: 14px; border-radius: 4px;" }
                            Skeleton { style: "width: 100px; height: 12px; border-radius: 4px;" }
                        }
                    }
                    Skeleton { style: "width: 100%; height: 80px; border-radius: 8px;" }
                }
            }

            Section { title: "Like Button",
                div { class: "demo-row demo-row-gap",
                    span { class: "demo-muted", "unliked:" }
                    LikeButton { object_ap_id: "https://jogga.fit/notes/like-demo-1".to_string(), token: token.clone(), initial_liked: false, initial_count: 5, like_fn, unlike_fn }
                    span { class: "demo-muted", "liked:" }
                    LikeButton { object_ap_id: "https://jogga.fit/notes/like-demo-2".to_string(), token: token.clone(), initial_liked: true, initial_count: 12, like_fn, unlike_fn }
                    span { class: "demo-muted", "no token:" }
                    LikeButton { object_ap_id: "https://jogga.fit/notes/like-demo-3".to_string(), token: None, initial_liked: false, initial_count: 0, like_fn, unlike_fn }
                }
            }

            Section { title: "Post Menu",
                div { class: "demo-row demo-row-gap",
                    span { class: "demo-muted", "owner + edit:" }
                    PostMenu { deleting: false, on_delete: move |_| {}, on_edit: Some(EventHandler::new(|_| {})) }
                    span { class: "demo-muted", "deleting…:" }
                    PostMenu { deleting: true, on_delete: move |_| {} }
                }
            }

            Section { title: "Exercise Stats Grid",
                ExerciseStatsGrid { distance_m: Some(10100.0), duration_s: Some(3240), pace_s_per_km: Some(320.8), elevation_gain_m: Some(85.0), avg_heart_rate_bpm: Some(158), max_heart_rate_bpm: Some(174), avg_cadence_rpm: Some(172.0) }
            }

            Section { title: "Exercise Stats Grid · Cycling (power data)",
                ExerciseStatsGrid { distance_m: Some(65000.0), duration_s: Some(7200), elevation_gain_m: Some(520.0), avg_heart_rate_bpm: Some(142), max_heart_rate_bpm: Some(168), avg_power_w: Some(210.0), max_power_w: Some(580.0), normalized_power_w: Some(225.0), avg_cadence_rpm: Some(88.0) }
            }

            Section { title: "Stat Toggle Chips",
                StatToggleChips { has_map: true, stats: STAT_TOGGLES.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(), hidden_stats }
                if !hidden_stats.read().is_empty() {
                    p { class: "demo-muted", "hidden: {hidden_stats.read():?}" }
                }
            }

            Section { title: "Media Collage (no media)",
                MediaCollage { route_url: None, image_urls: vec![], token: None, route_fn }
            }

            Section { title: "Carousel Overlay",
                button { class: "demo-btn", onclick: move |_| show_carousel.set(true),
                    i { class: "ph ph-images" } " Open Carousel (3 images)"
                }
                if *show_carousel.read() {
                    CarouselOverlay {
                        route_url: None,
                        image_urls: vec!["https://picsum.photos/seed/run1/800/600".to_string(), "https://picsum.photos/seed/run2/800/600".to_string(), "https://picsum.photos/seed/run3/800/600".to_string()],
                        token: None, initial_index: 0,
                        on_close: move |_| show_carousel.set(false),
                        route_fn,
                    }
                }
            }

            Section { title: "Route Map (mock GeoJSON)",
                RouteMap { route_url: "https://demo.internal/route/mock.geojson".to_string(), token: token.clone(), interactive: false, route_fn }
            }

            Section { title: "Feed Card · Run Exercise (owner)",
                FeedCard { item: run_item(), token: token.clone(), delete_fn, like_fn, unlike_fn, route_fn, update_fn }
            }
            Section { title: "Feed Card · Ride via Club (not owner)",
                FeedCard { item: ride_item(), token: token.clone(), delete_fn, like_fn, unlike_fn, route_fn, update_fn }
            }
            Section { title: "Feed Card · Swim (federated, hidden HR stat)",
                FeedCard { item: swim_item(), token: token.clone(), delete_fn, like_fn, unlike_fn, route_fn, update_fn }
            }

            Section { title: "Exercise Card (run)",
                ExerciseCard {
                    item: run_item(), token: token.clone(),
                    on_deleted: move |_| {}, on_edit_open: move |_| show_edit_modal.set(true),
                    delete_fn, like_fn, unlike_fn, route_fn,
                }
            }

            Section { title: "Edit Post Modal",
                button { class: "demo-btn", onclick: move |_| show_edit_modal.set(true),
                    i { class: "ph ph-pencil" } " Open Edit Modal"
                }
                if *show_edit_modal.read() {
                    div { style: "position:fixed;inset:0;background:rgba(0,0,0,.6);z-index:999;display:flex;align-items:center;justify-content:center;padding:16px;",
                        EditPostModal {
                            item: run_item(), token: "demo-token-abc".to_string(),
                            update_fn,
                            on_saved: move |_| show_edit_modal.set(false),
                            on_cancel: move |_| show_edit_modal.set(false),
                        }
                    }
                }
            }

            Section { title: "Reply Item + Reply Composer",
                ReplyItem { item: reply_item(), token: token.clone(), on_deleted: move |_| {}, delete_fn, like_fn, unlike_fn }
                ReplyItem { item: reply_item_own(), token: token.clone(), on_deleted: move |_| {}, delete_fn, like_fn, unlike_fn }
                ReplyComposer { in_reply_to: "https://jogga.fit/exercises/run-1".to_string(), token: "demo-token-abc".to_string(), on_posted: move |_| {}, create_reply_fn: reply_fn }
            }
        }
    }
}

#[component]
fn Section(title: String, children: Element) -> Element {
    rsx! {
        section { class: "demo-section",
            h2 { class: "demo-section-title", "{title}" }
            div { class: "demo-section-body", {children} }
        }
    }
}

use dioxus::prelude::*;

#[css_module("/src/pages/settings/style.css")]
struct Styles;

use crate::{
    AddAliasFn, AliasArgs, DeleteAccountFn, MoveAccountArgs, MoveAccountFn, PrivacySettingsArgs,
    RemoveAliasFn, SetPrivacySettingsFn, SetThemeArgs, SetThemeFn,
    components::{error_banner::ErrorBanner, setting_toggle::SettingToggle, theme_card::ThemeCard},
    types::{AuthSignal, MeResult, MigrationModalSignal, Theme, ThemeSignal},
};

#[component]
pub fn SettingsPageView(
    profile: Option<Result<MeResult, String>>,
    set_theme_fn: SetThemeFn,
    set_privacy_settings_fn: SetPrivacySettingsFn,
    delete_account_fn: DeleteAccountFn,
    on_theme_saved: EventHandler<Theme>,
    on_account_deleted: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "page-content",
                h1 { class: "settings-title", "Settings" }
                match profile {
                    None => rsx! { div { class: "loading-spinner", "Loading…" } },
                    Some(Err(_)) => rsx! { ErrorBanner { message: "Could not load settings. Please try again.".to_string() } },
                    Some(Ok(profile)) => rsx! {
                        AppearanceSection { profile: profile.clone(), set_theme_fn, on_theme_saved }
                        PrivacySection { profile: profile.clone(), set_privacy_settings_fn }
                        IntegrationsSection {}
                        MigrationRow { profile: profile.clone() }

                        DangerZoneSection { username: profile.username.clone(), delete_account_fn, on_account_deleted }
                    },
                }
            }
    }
}

#[component]
fn AppearanceSection(
    profile: MeResult,
    set_theme_fn: SetThemeFn,
    on_theme_saved: EventHandler<Theme>,
) -> Element {
    let auth = use_context::<AuthSignal>();
    let mut theme_signal = use_context::<ThemeSignal>();
    let token = auth
        .read()
        .as_ref()
        .map(|u| u.token.clone())
        .unwrap_or_default();

    let mut current_pref = use_signal(|| *theme_signal.peek());
    let mut saving = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    let make_pick = move |theme: Theme| {
        let token = token.clone();
        move |_: MouseEvent| {
            let prev = *current_pref.read();
            if theme == prev {
                return;
            }
            current_pref.set(theme);
            saving.set(true);
            error.set(None);
            let token = token.clone();
            spawn(async move {
                match set_theme_fn.call(SetThemeArgs { token, theme }).await {
                    Ok(()) => {
                        on_theme_saved.call(theme);
                        theme_signal.set(theme);
                    }
                    Err(e) => {
                        current_pref.set(prev);
                        error.set(Some(e));
                    }
                }
                saving.set(false);
            });
        }
    };

    let pref = *current_pref.read();
    let disabled = *saving.read();

    rsx! {
        section { class: Styles::settings_section, "data-testid": "settings-section",
            h2 { class: Styles::settings_section_title, "Appearance" }
            p { class: Styles::settings_section_desc, "Synced across all your devices." }

            div { class: Styles::theme_picker,
                ThemeCard {
                    id: "system",
                    label: "System",
                    active: matches!(pref, Theme::System),
                    disabled,
                    onclick: make_pick(Theme::System),
                    div { class: "{Styles::theme_preview} {Styles::theme_preview__system}",
                        div { class: Styles::theme_preview_topbar }
                        div { class: Styles::theme_preview_body,
                            div { class: Styles::theme_preview_card }
                            div { class: "{Styles::theme_preview_card} {Styles::theme_preview_card__sm}" }
                        }
                    }
                }
                ThemeCard {
                    id: "light",
                    label: "Light",
                    active: matches!(pref, Theme::Light),
                    disabled,
                    onclick: make_pick(Theme::Light),
                    div { class: "{Styles::theme_preview} {Styles::theme_preview__light}",
                        div { class: Styles::theme_preview_topbar }
                        div { class: Styles::theme_preview_body,
                            div { class: Styles::theme_preview_card }
                            div { class: "{Styles::theme_preview_card} {Styles::theme_preview_card__sm}" }
                        }
                    }
                }
                ThemeCard {
                    id: "dark",
                    label: "Dark",
                    active: matches!(pref, Theme::Dark),
                    disabled,
                    onclick: make_pick(Theme::Dark),
                    div { class: "{Styles::theme_preview} {Styles::theme_preview__dark}",
                        div { class: Styles::theme_preview_topbar }
                        div { class: Styles::theme_preview_body,
                            div { class: Styles::theme_preview_card }
                            div { class: "{Styles::theme_preview_card} {Styles::theme_preview_card__sm}" }
                        }
                    }
                }
            }

            if let Some(err) = error.read().as_ref() {
                ErrorBanner { message: err.clone() }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct Integration {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    icon: &'static str,
}

const INTEGRATIONS: &[Integration] = &[
    Integration {
        id: "amazfit",
        name: "Amazfit / Zepp",
        description: "Cloud sync via the Zepp Health API.",
        icon: "ph-watch",
    },
    Integration {
        id: "garmin",
        name: "Garmin Connect",
        description: "Cloud sync via OAuth and activity webhooks.",
        icon: "ph-watch",
    },
    Integration {
        id: "strava",
        name: "Strava",
        description: "Cloud sync via OAuth and webhooks.",
        icon: "ph-bicycle",
    },
    Integration {
        id: "apple-health",
        name: "Apple Health",
        description: "Import workouts and health metrics from Apple devices.",
        icon: "ph-apple-logo",
    },
    Integration {
        id: "wahoo",
        name: "Wahoo",
        description: "Sync rides and training sessions from Wahoo.",
        icon: "ph-lightning",
    },
];

#[component]
fn IntegrationsSection() -> Element {
    rsx! {
        section { class: Styles::settings_section, "data-testid": "settings-section",
            h2 { class: Styles::settings_section_title, "Integrations" }
            p { class: Styles::settings_section_desc,
                "Cloud sync integrations are coming soon."
            }
            div { class: Styles::integrations_grid,
                {INTEGRATIONS.iter().map(|i| {
                    rsx! { IntegrationCard { key: "{i.id}", integration: i.clone() } }
                })}
            }
        }
    }
}

#[component]
fn IntegrationCard(integration: Integration) -> Element {
    rsx! {
        div { class: format!("{} card", Styles::integration_card),
            div { class: Styles::integration_icon, i { class: "ph {integration.icon}" } }
            div { class: Styles::integration_info,
                span { class: Styles::integration_name, "{integration.name}" }
                span { class: Styles::integration_desc, "{integration.description}" }
            }
            span { class: Styles::badge_coming_soon, "Coming soon" }
        }
    }
}

#[component]
fn DangerZoneSection(
    username: String,
    delete_account_fn: DeleteAccountFn,
    on_account_deleted: EventHandler<()>,
) -> Element {
    let auth = use_context::<AuthSignal>();
    let token = auth
        .read()
        .as_ref()
        .map(|u| u.token.clone())
        .unwrap_or_default();

    let mut confirming = use_signal(|| false);
    let mut confirm_input = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    let on_delete = move |_: Event<MouseData>| {
        let token = token.clone();
        loading.set(true);
        error.set(None);
        spawn(async move {
            match delete_account_fn.call(token).await {
                Ok(()) => on_account_deleted.call(()),
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        section { class: format!("{} {}", Styles::settings_section, Styles::settings_danger_zone), "data-testid": "settings-danger-zone",
            h2 { class: Styles::settings_section_title, "Danger zone" }

            if !*confirming.read() {
                div { class: Styles::danger_row,
                    div { class: Styles::danger_info,
                        span { class: Styles::danger_label, "Delete account" }
                        span { class: Styles::danger_desc,
                            "Permanently delete your account and all your data. This cannot be undone."
                        }
                    }
                    button {
                        class: "btn btn-danger",
                        onclick: move |_| {
                            confirm_input.set(String::new());
                            error.set(None);
                            confirming.set(true);
                        },
                        "Delete account"
                    }
                }
            } else {
                div { class: Styles::danger_confirm, "data-testid": "danger-confirm",
                    p { class: Styles::danger_confirm_prompt,
                        "Type " strong { "@{username}" } " to confirm deletion."
                    }
                    div { class: "form-group",
                        input {
                            id: "delete-confirm",
                            r#type: "text",
                            placeholder: "@{username}",
                            autocomplete: "off",
                            value: "{confirm_input}",
                            oninput: move |e| confirm_input.set(e.value()),
                        }
                    }
                    if let Some(err) = error.read().as_ref() {
                        ErrorBanner { message: err.clone() }
                    }
                    div { class: Styles::danger_confirm_actions, "data-testid": "danger-confirm-actions",
                        button {
                            class: "btn btn-danger",
                            disabled: *loading.read() || *confirm_input.read() != format!("@{username}"),
                            onclick: on_delete,
                            if *loading.read() { "Deleting…" } else { "Permanently delete account" }
                        }
                        button {
                            class: "btn btn-ghost",
                            disabled: *loading.read(),
                            onclick: move |_| confirming.set(false),
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PrivacySection(profile: MeResult, set_privacy_settings_fn: SetPrivacySettingsFn) -> Element {
    let auth = use_context::<AuthSignal>();
    let token = auth
        .read()
        .as_ref()
        .map(|u| u.token.clone())
        .unwrap_or_default();

    let mut public_profile = use_signal(|| profile.public_profile);
    let mut saving = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    let on_toggle_public = {
        let token = token.clone();
        move |_| {
            let next_pub = !*public_profile.read();
            public_profile.set(next_pub);
            saving.set(true);
            error.set(None);
            let token = token.clone();
            spawn(async move {
                if let Err(e) = set_privacy_settings_fn
                    .call(PrivacySettingsArgs {
                        token,
                        public_profile: next_pub,
                    })
                    .await
                {
                    public_profile.set(!next_pub);
                    error.set(Some(e));
                }
                saving.set(false);
            });
        }
    };

    rsx! {
        section { class: Styles::settings_section, "data-testid": "settings-section",
            h2 { class: Styles::settings_section_title, "Privacy" }
            p { class: Styles::settings_section_desc, "Control who can see your profile and activity data." }

            SettingToggle {
                label: "Public profile".to_string(),
                description: "Anyone can view your profile and activities.".to_string(),
                checked: *public_profile.read(),
                disabled: *saving.read(),
                onchange: on_toggle_public,
            }

            if let Some(err) = error.read().as_ref() {
                ErrorBanner { message: err.clone() }
            }
        }
    }
}

#[component]
fn MigrationRow(profile: MeResult) -> Element {
    let mut modal_signal = use_context::<MigrationModalSignal>();
    let migrated = profile.moved_to.clone();

    rsx! {
        section { class: Styles::settings_section, "data-testid": "settings-section",
            div { class: Styles::settings_migration_row,
                div { class: Styles::settings_migration_info,
                    span { class: Styles::settings_migration_label,
                        i { class: "ph ph-arrow-square-right" }
                        " Account migration"
                    }
                    if let Some(target) = migrated.as_ref() {
                        span { class: Styles::settings_migration_desc, "Your account has been moved." }
                        div { class: Styles::migration_moved_badge,
                            i { class: "ph ph-arrow-right" }
                            a {
                                href: "{target}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "{target}"
                            }
                        }
                    } else {
                        span { class: Styles::settings_migration_desc,
                            "Move your followers to another ActivityPub instance."
                        }
                    }
                }
                button {
                    class: "btn btn-secondary btn-sm",
                    "data-testid": "migration-manage-btn",
                    onclick: move |_| modal_signal.set(Some(profile.clone())),
                    if migrated.is_some() { "View" } else { "Manage" }
                }
            }
        }
    }
}

#[component]
pub fn MigrationModal(
    profile: MeResult,
    on_close: EventHandler<()>,
    add_alias_fn: AddAliasFn,
    remove_alias_fn: RemoveAliasFn,
    move_account_fn: MoveAccountFn,
) -> Element {
    let auth = use_context::<AuthSignal>();
    let token = auth
        .read()
        .as_ref()
        .map(|u| u.token.clone())
        .unwrap_or_default();

    let migrated = profile.moved_to.clone();
    let aliases = use_signal(|| profile.also_known_as.clone());

    // Wizard step: 1 = add alias, 2 = move account
    let mut step = use_signal(|| {
        if profile.also_known_as.is_empty() {
            1u8
        } else {
            2u8
        }
    });

    rsx! {
        div {
            class: "modal-backdrop",
            onclick: move |_| on_close.call(()),

            div {
                class: Styles::migration_modal_card,
                onclick: move |e| e.stop_propagation(),

                div { class: Styles::migration_modal_header,
                    span { class: Styles::migration_modal_title,
                        i { class: "ph ph-arrow-square-right" }
                        " Account migration"
                    }
                    button {
                        class: "modal-close",
                        aria_label: "Close",
                        onclick: move |_| on_close.call(()),
                        i { class: "ph ph-x" }
                    }
                }

                div { class: Styles::migration_modal_body, "data-testid": "migration-section",
                    if let Some(target) = migrated.as_ref() {
                        div { class: Styles::migration_success, "data-testid": "migration-success",
                            span { class: Styles::migration_success_title,
                                i { class: "ph ph-check-circle" }
                                " Account migrated"
                            }
                            p { "Your followers have been redirected to your new account." }
                            div { class: Styles::migration_moved_badge, "data-testid": "migration-moved-to",
                                i { class: "ph ph-arrow-right" }
                                a {
                                    href: "{target}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "{target}"
                                }
                            }
                        }
                    } else {
                        // Wizard step indicators
                        div { class: Styles::wizard_steps_bar,
                            div {
                                class: if *step.read() == 1 { format!("{} active", Styles::wizard_step) } else { format!("{} done", Styles::wizard_step) },
                                "data-testid": "migration-step-alias",
                                role: "button",
                                onclick: move |_| step.set(1),
                                div { class: Styles::wizard_step_circle,
                                    if *step.read() > 1 {
                                        i { class: "ph ph-check" }
                                    } else {
                                        "1"
                                    }
                                }
                                span { class: Styles::wizard_step_label, "Add alias" }
                            }
                            div { class: Styles::wizard_step_connector }
                            div {
                                class: if *step.read() == 2 { format!("{} active", Styles::wizard_step) } else { format!("{} inactive", Styles::wizard_step) },
                                "data-testid": "migration-step-move",
                                div { class: Styles::wizard_step_circle, "2" }
                                span { class: Styles::wizard_step_label, "Move account" }
                            }
                        }

                        if *step.read() == 1 {
                            AliasesSubsection {
                                token: token.clone(),
                                aliases,
                                add_alias_fn,
                                remove_alias_fn,
                            }
                            div { class: Styles::wizard_step_actions,
                                button {
                                    class: "btn btn-primary btn-sm",
                                    disabled: aliases.read().is_empty(),
                                    onclick: move |_| step.set(2),
                                    "Next: Move Account"
                                    i { class: "ph ph-arrow-right" }
                                }
                            }
                        } else {
                            MoveAccountSubsection {
                                token: token.clone(),
                                username: profile.username.clone(),
                                has_alias: !aliases.read().is_empty(),
                                move_account_fn,
                            }
                            div { class: format!("{} {}", Styles::wizard_step_actions, Styles::wizard_step_actions__back),
                                button {
                                    class: "btn btn-ghost btn-sm",
                                    onclick: move |_| step.set(1),
                                    i { class: "ph ph-arrow-left" }
                                    " Manage aliases"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AliasesSubsection(
    token: String,
    aliases: Signal<Vec<String>>,
    add_alias_fn: AddAliasFn,
    remove_alias_fn: RemoveAliasFn,
) -> Element {
    let mut input = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    let on_add = {
        let token = token.clone();
        move |_: Event<MouseData>| {
            let new_alias = input.read().trim().to_string();
            if new_alias.is_empty() {
                return;
            }
            let token = token.clone();
            let mut aliases = aliases;
            busy.set(true);
            error.set(None);
            spawn(async move {
                match add_alias_fn
                    .call(AliasArgs {
                        token,
                        alias: new_alias.clone(),
                    })
                    .await
                {
                    Ok(()) => {
                        aliases.write().push(new_alias);
                        input.set(String::new());
                    }
                    Err(e) => error.set(Some(e)),
                }
                busy.set(false);
            });
        }
    };

    rsx! {
        div { class: Styles::settings_subsection, "data-testid": "aliases-section",
            h3 { class: Styles::settings_subsection_title, "This account's aliases" }
            p { class: Styles::settings_section_desc,
                "List accounts on other instances that you own. Your new account must add "
                strong { "this" }
                " account as an alias there before migration will be accepted."
            }

            if aliases.read().is_empty() {
                p { class: Styles::settings_empty, "data-testid": "aliases-empty", "No aliases yet." }
            } else {
                ul { class: Styles::settings_list, "data-testid": "aliases-list",
                    for alias in aliases.read().iter().cloned() {
                        AliasRow {
                            key: "{alias}",
                            token: token.clone(),
                            alias: alias.clone(),
                            aliases: aliases,
                            remove_alias_fn,
                        }
                    }
                }
            }

            div { class: "form-group",
                label { class: "label", r#for: "alias-input", "Add alias" }
                input {
                    id: "alias-input",
                    "data-testid": "alias-input",
                    class: "input",
                    r#type: "text",
                    autocomplete: "off",
                    placeholder: "@user@other.example or https://other.example/users/me",
                    value: "{input}",
                    oninput: move |e| input.set(e.value()),
                }
            }
            if let Some(err) = error.read().as_ref() {
                ErrorBanner { message: err.clone() }
            }
            button {
                class: "btn btn-secondary btn-sm",
                "data-testid": "alias-add-btn",
                disabled: *busy.read() || input.read().trim().is_empty(),
                onclick: on_add,
                if *busy.read() { "Adding…" } else { "Add alias" }
            }
        }
    }
}

#[component]
fn AliasRow(
    token: String,
    alias: String,
    aliases: Signal<Vec<String>>,
    remove_alias_fn: RemoveAliasFn,
) -> Element {
    let mut busy = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    let on_remove = {
        let alias = alias.clone();
        move |_: Event<MouseData>| {
            let token = token.clone();
            let alias = alias.clone();
            let mut aliases = aliases;
            busy.set(true);
            error.set(None);
            spawn(async move {
                match remove_alias_fn
                    .call(AliasArgs {
                        token,
                        alias: alias.clone(),
                    })
                    .await
                {
                    Ok(()) => {
                        aliases.write().retain(|x| x != &alias);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        busy.set(false);
                    }
                }
            });
        }
    };

    rsx! {
        li { class: Styles::settings_list_row, "data-testid": "alias-row",
            span { class: Styles::settings_list_text, "{alias}" }
            button {
                class: "btn btn-ghost btn-sm",
                "data-testid": "alias-remove-btn",
                disabled: *busy.read(),
                onclick: on_remove,
                if *busy.read() { "Removing…" } else { "Remove" }
            }
            if let Some(err) = error.read().as_ref() {
                ErrorBanner { message: err.clone() }
            }
        }
    }
}

#[component]
fn MoveAccountSubsection(
    token: String,
    username: String,
    has_alias: bool,
    move_account_fn: MoveAccountFn,
) -> Element {
    let mut target = use_signal(String::new);
    let mut confirming = use_signal(|| false);
    let mut confirm_input = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);
    let mut success = use_signal(|| false);

    let on_move = move |_: Event<MouseData>| {
        let token = token.clone();
        let target_ap_id = target.read().trim().to_string();
        busy.set(true);
        error.set(None);
        spawn(async move {
            match move_account_fn
                .call(MoveAccountArgs {
                    token,
                    target_ap_id,
                })
                .await
            {
                Ok(()) => {
                    success.set(true);
                    confirming.set(false);
                }
                Err(e) => error.set(Some(e)),
            }
            busy.set(false);
        });
    };

    rsx! {
        div { class: format!("{} {}", Styles::settings_subsection, Styles::settings_danger_zone), "data-testid": "move-section",
            h3 { class: Styles::settings_subsection_title, "Move this account" }
            p { class: Styles::settings_section_desc,
                "Redirect your followers to another account on any ActivityPub instance. The destination must list this account in its aliases first. "
                strong { "Irreversible." }
            }

            if *success.read() {
                div { class: Styles::migration_success, "data-testid": "move-success",
                    span { class: Styles::migration_success_title,
                        i { class: "ph ph-paper-plane-right" }
                        " Move queued"
                    }
                    p { "Your followers are being redirected in the background." }
                }
            } else if !*confirming.read() {
                if !has_alias {
                    div { class: Styles::migration_callout,
                        i { class: format!("ph ph-warning {}", Styles::migration_callout_icon) }
                        span {
                            "Add at least one alias above first — the destination must list this account in its aliases."
                        }
                    }
                }
                div { class: "form-group",
                    label { class: "label", r#for: "move-target", "New account" }
                    input {
                        id: "move-target",
                        "data-testid": "move-target-input",
                        class: "input",
                        r#type: "text",
                        autocomplete: "off",
                        placeholder: "@user@new.example or https://new.example/users/me",
                        value: "{target}",
                        disabled: !has_alias,
                        oninput: move |e| target.set(e.value()),
                    }
                }
                if let Some(err) = error.read().as_ref() {
                    ErrorBanner { message: err.clone() }
                }
                button {
                    class: "btn btn-danger",
                    "data-testid": "move-start-btn",
                    disabled: !has_alias || target.read().trim().is_empty(),
                    onclick: move |_| {
                        confirm_input.set(String::new());
                        error.set(None);
                        confirming.set(true);
                    },
                    "Move account"
                }
            } else {
                div { class: Styles::danger_confirm, "data-testid": "move-confirm-dialog",
                    p { class: Styles::danger_confirm_prompt,
                        "Type " strong { "@{username}" } " to confirm. This cannot be undone."
                    }
                    div { class: "form-group",
                        input {
                            id: "move-confirm",
                            "data-testid": "move-confirm-input",
                            class: "input",
                            r#type: "text",
                            placeholder: "@{username}",
                            autocomplete: "off",
                            value: "{confirm_input}",
                            oninput: move |e| confirm_input.set(e.value()),
                        }
                    }
                    if let Some(err) = error.read().as_ref() {
                        ErrorBanner { message: err.clone() }
                    }
                    div { class: Styles::danger_confirm_actions,
                        button {
                            class: "btn btn-danger",
                            "data-testid": "move-confirm-btn",
                            disabled: *busy.read() || *confirm_input.read() != format!("@{username}"),
                            onclick: on_move,
                            if *busy.read() { "Moving…" } else { "Confirm move" }
                        }
                        button {
                            class: "btn btn-ghost",
                            "data-testid": "move-cancel-btn",
                            disabled: *busy.read(),
                            onclick: move |_| confirming.set(false),
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}

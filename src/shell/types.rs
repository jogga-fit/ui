#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AppShellNavPlacement {
    Primary,
    DesktopOnly,
    Account,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AppShellNavItem {
    pub key: String,
    pub label: String,
    pub href: String,
    pub icon_class: String,
    pub placement: AppShellNavPlacement,
}

impl AppShellNavItem {
    pub fn new(
        key: impl Into<String>,
        label: impl Into<String>,
        href: impl Into<String>,
        icon_class: impl Into<String>,
        placement: AppShellNavPlacement,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            href: href.into(),
            icon_class: icon_class.into(),
            placement,
        }
    }

    pub fn primary(
        key: impl Into<String>,
        label: impl Into<String>,
        href: impl Into<String>,
        icon_class: impl Into<String>,
    ) -> Self {
        Self::new(key, label, href, icon_class, AppShellNavPlacement::Primary)
    }

    pub fn desktop_only(
        key: impl Into<String>,
        label: impl Into<String>,
        href: impl Into<String>,
        icon_class: impl Into<String>,
    ) -> Self {
        Self::new(
            key,
            label,
            href,
            icon_class,
            AppShellNavPlacement::DesktopOnly,
        )
    }

    pub fn account(
        key: impl Into<String>,
        label: impl Into<String>,
        href: impl Into<String>,
        icon_class: impl Into<String>,
    ) -> Self {
        Self::new(key, label, href, icon_class, AppShellNavPlacement::Account)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AppShellUser {
    pub username: String,
    pub display_name: Option<String>,
}

impl AppShellUser {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            display_name: None,
        }
    }

    pub fn with_display_name(username: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            display_name: Some(display_name.into()),
        }
    }

    pub fn name(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.username)
    }

    pub fn initial(&self) -> String {
        self.name()
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "?".to_string())
    }
}

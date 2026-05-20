#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AppShellNavPlacement {
    Primary,
    User,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PageNav {
    pub key: &'static str,
    pub label: &'static str,
    pub href: &'static str,
    pub icon_class: &'static str,
    pub placement: AppShellNavPlacement,
}

impl PageNav {
    pub const fn primary(
        key: &'static str,
        label: &'static str,
        href: &'static str,
        icon_class: &'static str,
    ) -> Self {
        Self {
            key,
            label,
            href,
            icon_class,
            placement: AppShellNavPlacement::Primary,
        }
    }

    pub const fn user(
        key: &'static str,
        label: &'static str,
        href: &'static str,
        icon_class: &'static str,
    ) -> Self {
        Self {
            key,
            label,
            href,
            icon_class,
            placement: AppShellNavPlacement::User,
        }
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

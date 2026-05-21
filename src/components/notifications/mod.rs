use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[css_module("/src/components/notifications/style.css")]
pub struct Styles;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum NotificationKind {
    FollowRequest,
    NewFollower,
    FollowAccepted,
    Like,
}

impl NotificationKind {
    pub fn action_text(&self, item: &NotificationItem) -> String {
        match self {
            NotificationKind::Like => match &item.object_title {
                Some(title) => format!("liked your post - {title}"),
                None => "liked your post".to_string(),
            },
            NotificationKind::FollowRequest => "sent you a follow request".to_string(),
            NotificationKind::NewFollower => "started following you".to_string(),
            NotificationKind::FollowAccepted => "accepted your follow request".to_string(),
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            NotificationKind::Like => "ph-heart",
            NotificationKind::FollowRequest | NotificationKind::NewFollower => "ph-user-plus",
            NotificationKind::FollowAccepted => "ph-check-circle",
        }
    }

    pub fn color(&self) -> String {
        match self {
            NotificationKind::Like => Styles::notification_kind_like,
            NotificationKind::FollowRequest | NotificationKind::NewFollower => {
                Styles::notification_kind_follow
            }
            NotificationKind::FollowAccepted => Styles::notification_kind_accepted,
        }
        .to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NotificationItem {
    pub id: String,
    pub kind: NotificationKind,
    pub from_ap_id: String,
    pub from_username: String,
    pub from_display_name: Option<String>,
    pub from_avatar_url: Option<String>,
    /// Populated for "like" notifications.
    pub object_ap_id: Option<String>,
    pub object_title: Option<String>,
    pub is_read: bool,
    pub created_at: String,
}

impl NotificationItem {
    pub fn initial(&self) -> String {
        self.from_display_name
            .as_deref()
            .unwrap_or(&self.from_username)
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "?".to_string())
    }
}

pub fn notification_time(created_at: &str) -> String {
    if let Some((date, time_str)) = created_at.split_once('T') {
        let hhmm: String = time_str.chars().take(5).collect();
        return format!("{date} {hhmm}");
    }
    created_at.to_string()
}

#[component]
pub fn NotificationList(
    notifications: Vec<NotificationItem>,
    #[props(default)] on_dismiss: Option<EventHandler<String>>,
) -> Element {
    if notifications.is_empty() {
        return rsx! {
            div { class: Styles::notification_empty,
                i { class: "ph ph-bell" }
                span { "No notifications yet" }
            }
        };
    }

    rsx! {
        for item in notifications {
            NotificationRow {
                key: "{item.id}",
                item,
                on_dismiss,
            }
        }
    }
}

#[component]
pub fn NotificationRow(
    item: NotificationItem,
    #[props(default)] on_dismiss: Option<EventHandler<String>>,
) -> Element {
    let item_id = item.id.clone();
    let initial = item.initial();
    let name = item
        .from_display_name
        .as_deref()
        .unwrap_or(&item.from_username)
        .to_owned();
    let action = item.kind.action_text(&item);
    let timestamp = notification_time(&item.created_at);
    let kind_icon = item.kind.icon();
    let kind_color = item.kind.color();

    let mut row_class = Styles::notification_item.to_string();
    if !item.is_read {
        row_class.push(' ');
        row_class.push_str(&Styles::notification_item_unread);
    }

    rsx! {
        div { class: row_class,
            div { class: Styles::notification_item_avatar_wrap,
                div { class: Styles::notification_item_avatar, "{initial}" }
                span {
                    class: format!("{} {}", Styles::notification_kind_icon, kind_color),
                    i { class: format!("ph {kind_icon}") }
                }
            }
            div { class: Styles::notification_body,
                span { class: Styles::notification_name, "{name}" }
                span { class: Styles::notification_text, " {action}" }
                span { class: Styles::notification_ts, "{timestamp}" }
            }
            button {
                class: Styles::notification_dismiss,
                title: "Dismiss",
                "aria-label": "Dismiss notification",
                onclick: move |event| {
                    event.stop_propagation();
                    if let Some(handler) = on_dismiss {
                        handler.call(item_id.clone());
                    }
                },
                i { class: "ph ph-x" }
            }
        }
    }
}

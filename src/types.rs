use dioxus::prelude::Signal;
use serde::{Deserialize, Deserializer, Serialize};

use crate::exercise::ExerciseType;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginResult {
    pub token: String,
    #[serde(default)]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterInitResult {
    pub otp_id: String,
    /// Only set in debug builds; always `None` in release.
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterVerifyResult {
    pub username: String,
    pub ap_id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtpVerifyResult {
    pub token: String,
    /// Set for registration OTPs; `None` for password reset.
    pub username: Option<String>,
    /// Set for registration OTPs; `None` for password reset.
    pub ap_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    System,
    Dark,
    Light,
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dark => write!(f, "dark"),
            Self::Light => write!(f, "light"),
            Self::System => write!(f, "system"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeResult {
    pub username: String,
    pub ap_id: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    #[serde(default)]
    pub is_admin: bool,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub show_in_directory: bool,
    pub public_profile: bool,
    pub theme: Theme,
    #[serde(default)]
    pub also_known_as: Vec<String>,
    #[serde(default)]
    pub moved_to: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FollowingItem {
    pub ap_id: String,
    pub username: String,
    pub domain: String,
    pub is_local: bool,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub accepted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ActorInfo {
    pub username: String,
    pub domain: String,
    pub ap_id: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub public_profile: bool,
    pub followers_count: i64,
    pub following_count: i64,
}

/// The ActivityPub object type carried in a feed item.
///
/// Variants are serialized/deserialized as their PascalCase names.
/// Any unrecognised string from a federated server is kept verbatim in
/// `Unknown(String)` so the app never panics on forward-compat types.
#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum ObjectType {
    Exercise,
    Note,
    /// Forward-compatibility catch-all for AP types we don't recognise yet.
    Unknown(String),
}

impl<'de> Deserialize<'de> for ObjectType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Exercise" => ObjectType::Exercise,
            "Note" => ObjectType::Note,
            _ => ObjectType::Unknown(s),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ExerciseStats {
    pub exercise_type: Option<ExerciseType>,
    pub duration_s: Option<i64>,
    pub distance_m: Option<f64>,
    pub elevation_gain_m: Option<f64>,
    pub avg_heart_rate_bpm: Option<i32>,
    pub max_heart_rate_bpm: Option<i32>,
    pub avg_power_w: Option<f64>,
    pub max_power_w: Option<f64>,
    pub normalized_power_w: Option<f64>,
    pub avg_cadence_rpm: Option<f64>,
    pub avg_pace_s_per_km: Option<f64>,
    pub device: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FeedItem {
    pub id: String,
    /// The AP ID of the object (Note/Exercise) — used for like/unlike actions.
    pub object_ap_id: String,
    pub actor_username: String,
    pub actor_domain: String,
    pub actor_display_name: Option<String>,
    pub actor_is_local: bool,
    pub actor_ap_id: String,
    pub actor_avatar_url: Option<String>,
    pub activity_type: String,
    pub object_type: ObjectType,
    pub content: Option<String>,
    pub published: String,
    #[serde(flatten)]
    pub stats: ExerciseStats,
    pub title: Option<String>,
    pub image_urls: Vec<String>,
    pub like_count: i64,
    pub viewer_has_liked: bool,
    pub viewer_is_owner: bool,
    pub reply_count: i64,
    /// AP ID of the parent object, if this is a reply.
    pub in_reply_to: Option<String>,
    /// Route GeoJSON endpoint URL — present for Exercise objects that have a recorded route.
    pub route_url: Option<String>,
    /// Stats currently hidden from display (Exercise only).
    pub hidden_stats: Vec<String>,
    /// Set when this post was shown via a club Announce.
    pub via_club_handle: Option<String>,
    /// Human-readable club display name.
    pub via_club_display: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConnectionItem {
    pub ap_id: String,
    pub username: String,
    pub domain: String,
    pub is_local: bool,
    pub display_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConnectionsResult {
    /// false when the profile is private and the viewer is not an authorized follower.
    pub visible: bool,
    pub following: Vec<ConnectionItem>,
    pub followers: Vec<ConnectionItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthUser {
    pub token: String,
    pub username: String,
    #[serde(default)]
    pub ap_id: String,
}

/// Global auth state. `None` = not logged in.
pub type AuthSignal = Signal<Option<AuthUser>>;

/// Global theme signal.
pub type ThemeSignal = Signal<Theme>;

/// Migration modal — `Some(profile)` while open, `None` while closed.
pub type MigrationModalSignal = Signal<Option<MeResult>>;


use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::channel::channel::Channel;

use super::{emoji::Emoji, role::Role, sticker::Sticker};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub home_header: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub features: Vec<String>,
    pub banner: Option<String>,
    pub owner_id: String,
    pub application_id: Option<String>,
    pub region: String,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: i32,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: i32,
    pub widget_enabled: bool,
    pub widget_channel_id: Option<String>,
    pub verification_level: i32,
    pub roles: Vec<Role>,
    pub default_message_notifications: i32,
    pub mfa_level: i32,
    pub explicit_content_filter: i32,
    pub max_presences: Option<i32>,
    pub max_members: i32,
    pub max_video_channel_users: i32,
    pub vanity_url_code: Option<String>,
    pub premium_tier: i32,
    pub premium_subscription_count: i32,
    pub preferred_locale: String,
    pub rules_channel_id: Option<String>,
    pub safety_alerts_channel_id: Option<String>,
    pub public_updates_channel_id: Option<String>,
    pub hub_type: Option<String>,
    pub premium_progress_bar_enabled: bool,
    pub latest_onboarding_question_id: Option<String>,
    pub nsfw: bool,
    pub nsfw_level: i32,
    pub emojis: Vec<Emoji>,
    pub stickers: Vec<Sticker>,
    pub incidents_data: Option<HashMap<String, String>>,
    pub inventory_settings: Option<String>,
    pub embed_enabled: bool,
    pub embed_channel_id: Option<String>,
    pub channels: Option<Vec<Channel>>
}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{custom_status::CustomStatus, guild_folder::GuildFolder};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub locale: String,
    pub show_current_game: bool,
    pub restricted_guilds: Vec<String>,
    pub default_guilds_restricted: bool,
    pub inline_attachment_media: bool,
    pub inline_embed_media: bool,
    pub gif_auto_play: bool,
    pub render_embeds: bool,
    pub render_reactions: bool,
    pub animate_emoji: bool,
    pub enable_tts_command: bool,
    pub message_display_compact: bool,
    pub convert_emoticons: bool,
    pub explicit_content_filter: i32,
    pub disable_games_tab: bool,
    pub theme: String,
    pub developer_mode: bool,
    pub detect_platform_accounts: bool,
    pub status: String,
    pub afk_timeout: i32,
    pub timezone_offset: i32,
    pub stream_notifications_enabled: bool,
    pub allow_accessibility_detection: bool,
    pub contact_sync_enabled: bool,
    pub native_phone_integration_enabled: bool,
    pub animate_stickers: i32,
    pub friend_discovery_flags: i32,
    pub view_nsfw_guilds: bool,
    pub view_nsfw_commands: bool,
    pub passwordless: bool,
    pub friend_source_flags: HashMap<String, bool>,
    pub guild_folders: Vec<GuildFolder>,
    pub custom_status: CustomStatus,
    pub activity_restricted_guild_ids: Vec<String>,
    pub activity_joining_restricted_guild_ids: Vec<String>,
    pub broadcast_allow_friends: bool,
    pub broadcast_allowed_guild_ids: Vec<String>,
    pub broadcast_allowed_user_ids: Vec<String>
}
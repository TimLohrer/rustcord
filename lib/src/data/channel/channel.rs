use serde::{Deserialize, Serialize};

use crate::data::discord::permission_overwrite::PermissionOverwrite;

use super::{available_tag::AvailableTag, default_reaction_emoji::DefaultReactionEmoji, icon_emoji::IconEmoji};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub id: String,
    pub r#type: i32,
    pub name: String,
	pub flags: i32,
    pub guild_id: String,
    pub parent_id: Option<String>,
    pub children: Option<Vec<Channel>>,
    pub position: i32,
    pub video_quality_mode: Option<i32>,
	pub bitrate: Option<i32>,
	pub user_limit: Option<i32>,
	pub rtc_region: Option<String>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub last_message_id: Option<String>,
    pub topic: Option<String>,
    pub rate_limit_per_user: Option<i32>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub default_auto_archive_duration: Option<i32>,
	pub available_tags: Option<Vec<AvailableTag>>,
	pub default_reaction_emoji: Option<DefaultReactionEmoji>,
	pub default_sort_order: Option<i32>,
	pub default_forum_layout: Option<i32>,
	pub icon_emoji: Option<IconEmoji>,
	pub theme_color: Option<i32>,
	pub template: Option<String>,
    pub voice_background_display: Option<String>,
    pub nsfw: Option<bool>
}
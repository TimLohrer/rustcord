use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Thread {
    pub id: String,
    pub r#type: i64,
    pub last_message_id: String,
    pub flags: i64,
    pub guild_id: String,
    pub name: String,
    pub parent_id: String,
    pub rate_limit_per_user: i64,
    pub bitrate: i64,
    pub user_limit: i64,
    pub rtc_region: Option<String>,
    pub owner_id: String,
    pub thread_metadata: ThreadMetadata,
    pub message_count: i64,
    pub member_count: i64,
    pub total_message_sent: i64,
    pub member_ids_preview: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub archive_timestamp: String,
    pub auto_archive_duration: i64,
    pub locked: bool,
    pub create_timestamp: String,
}
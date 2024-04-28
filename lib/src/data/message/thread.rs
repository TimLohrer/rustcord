use serde::{Deserialize, Serialize};

use super::thread_metadata::ThreadMetadata;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Thread {
    id: String,
    #[serde(rename = "type")]
    thread_type: i64,
    last_message_id: String,
    flags: i64,
    guild_id: String,
    name: String,
    parent_id: String,
    rate_limit_per_user: i64,
    bitrate: i64,
    user_limit: i64,
    rtc_region: Option<String>,
    owner_id: String,
    thread_metadata: ThreadMetadata,
    message_count: i64,
    member_count: i64,
    total_message_sent: i64,
    member_ids_preview: Vec<String>,
}
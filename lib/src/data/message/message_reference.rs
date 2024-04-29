use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageReference {
    pub channel_id: String,
    pub message_id: Option<String>,
    pub guild_id: String,
}
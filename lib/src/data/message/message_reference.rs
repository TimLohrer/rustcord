use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageReference {
    channel_id: String,
    message_id: Option<String>,
    guild_id: String,
}
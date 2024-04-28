use serde::{Deserialize, Serialize};

use super::message_emoji::MessageEmoji;

#[derive(Serialize, Deserialize)]
pub struct MessageComponent {
    #[serde(rename = "type")]
    component_type: i64,
    custom_id: String,
    style: i64,
    label: String,
    emoji: MessageEmoji,
}
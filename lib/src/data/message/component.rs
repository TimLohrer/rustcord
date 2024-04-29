use serde::{Deserialize, Serialize};

use super::message_emoji::MessageEmoji;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComponentRow {
    pub r#type: i64,
    pub components: Vec<Component>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Component {
    pub r#type: i64,
    pub custom_id: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub style: Option<i64>,
    pub min_values: Option<i64>,
    pub max_values: Option<i64>,
    pub emoji: Option<MessageEmoji>,
    pub options: Option<Vec<ComponentDropdownOption>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComponentDropdownOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<MessageEmoji>,
}
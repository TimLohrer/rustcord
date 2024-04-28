use serde::{Deserialize, Serialize};

use super::message_user::MessageUser;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interaction {
    id: String,
    #[serde(rename = "type")]
    interaction_type: i64,
    name: String,
    user: MessageUser,
}
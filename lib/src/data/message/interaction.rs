use serde::{Deserialize, Serialize};

use super::message_user::MessageUser;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interaction {
    pub id: String,
    pub r#type: i64,
    pub name: String,
    pub user: MessageUser,
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageEmoji {
    pub id: String,
    pub name: String,
}
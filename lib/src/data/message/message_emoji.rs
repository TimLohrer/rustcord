use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageEmoji {
    id: String,
    name: String,
}
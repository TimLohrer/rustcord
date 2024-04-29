use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StickerItem {
    pub id: String,
    pub name: String,
    pub format_type: i64,
}
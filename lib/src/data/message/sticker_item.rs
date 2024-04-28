use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StickerItem {
    id: String,
    name: String,
    format_type: i64,
}
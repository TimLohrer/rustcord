use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StickerItem {
    id: String,
    name: String,
    format_type: i64,
}
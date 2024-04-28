use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvatarDecorationData {
    pub asset: String,
    pub sku_id: String
}
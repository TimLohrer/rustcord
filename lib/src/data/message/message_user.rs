use serde::{Deserialize, Serialize};

use crate::data::discord::avatar_decoration::AvatarDecorationData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageUser {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: i64,
    pub flags: i64,
    pub banner: Option<String>,
    pub accent_color: Option<String>,
    pub global_name: Option<String>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
    pub banner_color: Option<String>,
    pub clan: Option<String>,
    pub bot: Option<bool>,
    pub verified: Option<bool>,
    pub system: Option<bool>,
}
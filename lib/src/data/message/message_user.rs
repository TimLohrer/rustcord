use serde::{Deserialize, Serialize};

use crate::data::discord::avatar_decoration::AvatarDecorationData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageUser {
    id: String,
    username: String,
    avatar: Option<String>,
    discriminator: String,
    public_flags: i64,
    flags: i64,
    banner: Option<String>,
    accent_color: Option<String>,
    global_name: Option<String>,
    avatar_decoration_data: Option<AvatarDecorationData>,
    banner_color: Option<String>,
    clan: Option<String>,
    bot: Option<bool>,
}
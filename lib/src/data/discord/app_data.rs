use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::{guild::{guild::Guild, guild_minimal::GuildMinimal}, message::message::Message};

use super::{settings::Settings, user::User};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppData {
    pub token: String,
    pub settings: Settings,
    pub user: User,
    pub guilds_minimal: Vec<GuildMinimal>,
    pub guilds: Vec<Guild>,
    pub message_cache: HashMap<String, Vec<Message>>
}

impl AppData {
    pub fn new() -> Self {
        Self {
            token: String::new(),
            settings: Settings::new(),
            user: User::new(),
            guilds_minimal: Vec::new(),
            guilds: Vec::new(),
            message_cache: HashMap::new()
        }
    }
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Discord {
    pub token: String,
    pub settings: Settings,
    pub user: User,
    pub guilds_minimal: Vec<GuildMinimal>,
    pub guilds: Vec<Guild>
}

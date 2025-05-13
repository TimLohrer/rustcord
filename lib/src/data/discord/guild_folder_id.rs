use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum GuildFolderId {
    String(String),
    Int(u32)
}

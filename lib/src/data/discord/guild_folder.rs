use serde::{Deserialize, Serialize};

use super::guild_folder_id::GuildFolderId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildFolder {
  pub guild_ids: Vec<String>,
  pub id: Option<GuildFolderId>,
  pub name: Option<String>,
  pub color: Option<i32>
}
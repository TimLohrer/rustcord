use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: String,
    pub position: i32,
    pub color: i32,
    pub hoist: bool,
    pub managed: bool,
    pub mentionable: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub tags: Option<HashMap<String, Option<String>>>,
    pub flags: i32
}
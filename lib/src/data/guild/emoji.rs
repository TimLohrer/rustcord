use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Emoji {
    pub id: String,
    pub name: String,
    pub roles: Vec<String>,
    pub require_colons: bool,
    pub managed: bool,
    pub animated: bool,
    pub available: bool
}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resolved {
    pub users: HashMap<String, String>,
    pub members: HashMap<String, String>,
    pub channels: HashMap<String, String>,
    pub roles: HashMap<String, String>,
}
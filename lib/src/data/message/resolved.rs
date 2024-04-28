use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resolved {
    users: HashMap<String, String>,
    members: HashMap<String, String>,
    channels: HashMap<String, String>,
    roles: HashMap<String, String>,
}
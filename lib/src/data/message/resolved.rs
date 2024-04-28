use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Resolved {
    users: Vec<String>,
    members: Vec<String>,
    channels: Vec<String>,
    roles: Vec<String>,
}
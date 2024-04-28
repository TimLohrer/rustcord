use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomStatus {
    pub text: String,
    pub expires_at: Option<String>,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    id: String,
    name: String,
    icon: String,
    description: String,
    #[serde(rename = "type")]
    application_type: Option<String>,
    cover_image: String,
    summary: String,
    is_monetized: bool,
}
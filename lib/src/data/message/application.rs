use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub description: Option<String>,
    pub r#type: Option<String>,
    pub cover_image: String,
    pub summary: String,
    pub is_monetized: bool,
}
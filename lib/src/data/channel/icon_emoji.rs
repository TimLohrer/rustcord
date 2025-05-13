use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IconEmoji {
  pub id: Option<String>,
  pub name: Option<String>
}
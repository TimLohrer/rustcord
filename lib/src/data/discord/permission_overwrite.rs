use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PermissionOverwrite {
	pub id: String,
	pub r#type: i32,
	pub allow: String,
	pub deny: String
}
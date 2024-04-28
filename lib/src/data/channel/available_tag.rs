use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvailableTag {
	pub id: String,
	pub name: String,
	pub moderated: bool,
	pub emoji_id: Option<String>,
	pub emoji_name: Option<String>
}
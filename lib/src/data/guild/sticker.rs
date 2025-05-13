use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sticker {
	pub id: String,
	pub name: String,
	pub tags: String,
	pub r#type: i32,
	pub format_type: i32,
	pub description: Option<String>,
	pub asset: Option<String>,
	pub available: bool,
	pub guild_id: String
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DefaultReactionEmoji {
	pub emoji_id: Option<String>,
	pub emoji_name: Option<String>
}
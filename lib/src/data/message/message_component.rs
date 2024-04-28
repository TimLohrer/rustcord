use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageComponent {
    #[serde(rename = "type")]
    component_type: i64,
    components: Vec<MessageComponent>,
}
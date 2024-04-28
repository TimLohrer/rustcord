use serde::{Deserialize, Serialize};

use super::{authorizing_integration_owner::AuthorizingIntegrationOwners, message_user::MessageUser};

#[derive(Serialize, Deserialize)]
pub struct InteractionMetadata {
    id: String,
    #[serde(rename = "type")]
    interaction_metadata_type: i64,
    user_id: String,
    user: MessageUser,
    authorizing_integration_owners: AuthorizingIntegrationOwners,
    name: String,
}
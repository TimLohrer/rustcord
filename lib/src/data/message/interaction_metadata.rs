use serde::{Deserialize, Serialize};

use super::{authorizing_integration_owner::AuthorizingIntegrationOwners, message_user::MessageUser};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InteractionMetadata {
    pub id: String,
    pub r#type: i64,
    pub user_id: String,
    pub user: MessageUser,
    pub authorizing_integration_owners: AuthorizingIntegrationOwners,
    pub name: String,
}
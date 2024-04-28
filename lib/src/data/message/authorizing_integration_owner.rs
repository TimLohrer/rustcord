use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizingIntegrationOwners {
    #[serde(rename = "0")]
    the_0: String,
}
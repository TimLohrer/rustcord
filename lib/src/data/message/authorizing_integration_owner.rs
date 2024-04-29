use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizingIntegrationOwners {
    #[serde(rename = "0")]
    pub the_0: String,
}
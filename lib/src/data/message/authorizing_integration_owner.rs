use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthorizingIntegrationOwners {
    #[serde(rename = "0")]
    the_0: String,
}
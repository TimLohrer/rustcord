use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IncidentsData {
    pub invites_disabled_until: Option<String>,
    pub dms_disabled_until: Option<String>,
}
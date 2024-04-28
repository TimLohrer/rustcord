use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ThreadMetadata {
    archived: bool,
    archive_timestamp: String,
    auto_archive_duration: i64,
    locked: bool,
    create_timestamp: String,
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub size: i64,
    pub url: String,
    pub proxy_url: String,
    pub content_type: String,
    pub content_scan_version: i64,
    pub duration_secs: Option<f64>,
    pub waveform: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub placeholder: Option<String>,
    pub placeholder_version: Option<i64>,
}
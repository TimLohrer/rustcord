use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Attachment {
    id: String,
    filename: String,
    size: i64,
    url: String,
    proxy_url: String,
    content_type: String,
    content_scan_version: i64,
    duration_secs: Option<f64>,
    waveform: Option<String>,
    width: Option<i64>,
    height: Option<i64>,
    placeholder: Option<String>,
    placeholder_version: Option<i64>,
}
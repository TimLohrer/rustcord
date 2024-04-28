use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Embed {
    #[serde(rename = "type")]
    embed_type: String,
    url: Option<String>,
    title: Option<String>,
    description: Option<String>,
    content_scan_version: Option<i64>,
    color: Option<i64>,
    fields: Option<Vec<EmbedField>>,
    provider: Option<EmbedProvider>,
    thumbnail: Option<EmbedThumbnail>,
    video: Option<EmbedThumbnail>,
    author: Option<EmbedAuthor>,
    footer: Option<EmbedFooter>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedAuthor {
    name: String,
    icon_url: String,
    proxy_icon_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedFooter {
    text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedProvider {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedThumbnail {
    url: String,
    proxy_url: String,
    width: i64,
    height: i64,
    placeholder: Option<String>,
    placeholder_version: Option<i64>,
}
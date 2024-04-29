use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Embed {
    pub r#type: String,
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content_scan_version: Option<i64>,
    pub color: Option<i64>,
    pub fields: Option<Vec<EmbedField>>,
    pub provider: Option<EmbedProvider>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedThumbnail>,
    pub author: Option<EmbedAuthor>,
    pub footer: Option<EmbedFooter>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedAuthor {
    pub name: String,
    pub icon_url: String,
    pub proxy_icon_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedFooter {
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedProvider {
    pub name: String,
    pub url: String,
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
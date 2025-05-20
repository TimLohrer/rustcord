use serde::{Deserialize, Serialize};

use super::gateway_dtos::{Message, OpCode};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct HeartbeatMessage {}

impl Message for HeartbeatMessage {
    fn op_code(&self) -> OpCode {
        OpCode::Heartbeat
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct IdentifyMessage {
    pub token: String,
    pub intents: u64,
    pub properties: IdentifyProperties,
}

impl Message for IdentifyMessage {
    fn op_code(&self) -> OpCode {
        OpCode::Identify
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct IdentifyProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct VoiceStateUpdateMessage {
    pub guild_id: String,
    pub channel_id: Option<String>,
    pub self_mute: bool,
    pub self_deaf: bool,
}

impl Message for VoiceStateUpdateMessage {
    fn op_code(&self) -> OpCode {
        OpCode::UpdateVoiceState
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RequestGuildMembersMessage {
    pub guild_id: String,
    pub query: Option<String>,
    pub limit: u64,
    pub presences: Option<bool>,
    pub user_ids: Option<Vec<String>>,
    pub nonce: Option<String>,
}

impl Message for RequestGuildMembersMessage {
    fn op_code(&self) -> OpCode {
        OpCode::RequestGuildMembers
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UpdatePresenceMessage {
    pub since: Option<u64>,
    pub activities: Vec<Activity>,
    pub status: String,
    pub afk: bool
}

impl Message for UpdatePresenceMessage {
    fn op_code(&self) -> OpCode {
        OpCode::UpdatePresence
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Activity {
    pub name: String,
    pub r#type: u8,
    pub url: Option<String>,
    pub created_at: u128,
    pub timestamps: Option<ActivityTimestamps>,
    pub application_id: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<ActivityEmoji>,
    pub party: Option<ActivityParty>,
    pub assets: Option<ActivityAssets>,
    pub secrets: Option<ActivitySecrets>,
    pub instance: bool,
    pub flags: Option<u64>,
    pub buttons: Option<Vec<ActivityButton>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ActivityTimestamps {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ActivityEmoji {
    pub name: String,
    pub id: Option<String>,
    pub animated: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ActivityParty {
    pub id: Option<String>,
    pub size: Option<Vec<u64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ActivityAssets {
    pub large_image: String,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ActivitySecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    pub match_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ActivityButton {
    pub label: String,
    pub url: String,
}
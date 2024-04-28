use serde::{Serialize, Deserialize};

use super::{activity_instance::ActivityInstance, application::Application, attachment::Attachment, embed::Embed, interaction::Interaction, interaction_metadata::InteractionMetadata, message_component::MessageComponent, message_reference::MessageReference, message_user::MessageUser, poll::Poll, resolved::Resolved, sticker_item::StickerItem, thread::Thread};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    id: String,
    #[serde(rename = "type")]
    discord_message_type: i64,
    content: String,
    channel_id: String,
    author: MessageUser,
    attachments: Vec<Attachment>,
    embeds: Vec<Embed>,
    mentions: Vec<Option<MessageUser>>,
    mention_roles: Vec<String>,
    pinned: bool,
    mention_everyone: bool,
    tts: bool,
    timestamp: String,
    edited_timestamp: Option<String>,
    flags: i64,
    components: Vec<MessageComponent>,
    webhook_id: Option<String>,
    message_reference: Option<MessageReference>,
    referenced_message: Option<MessageReference>,
    thread: Option<Thread>,
    position: Option<i64>,
    application: Option<Application>,
    application_id: Option<String>,
    interaction: Option<Interaction>,
    activity_instance: Option<ActivityInstance>,
    interaction_metadata: Option<InteractionMetadata>,
    poll: Option<Poll>,
    sticker_items: Option<Vec<StickerItem>>,
    resolved: Option<Resolved>,
}

use serde::{Serialize, Deserialize};

use super::{activity_instance::ActivityInstance, application::Application, attachment::Attachment, embed::Embed, interaction::Interaction, interaction_metadata::InteractionMetadata, component::ComponentRow, message_reference::MessageReference, message_user::MessageUser, poll::Poll, resolved::Resolved, sticker_item::StickerItem, thread::Thread};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub id: String,
    pub r#type: i64,
    pub content: String,
    pub channel_id: String,
    pub author: MessageUser,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub mentions: Vec<Option<MessageUser>>,
    pub mention_roles: Vec<String>,
    pub pinned: bool,
    pub mention_everyone: bool,
    pub tts: bool,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub flags: i64,
    pub components: Vec<ComponentRow>,
    pub webhook_id: Option<String>,
    pub message_reference: Option<MessageReference>,
    pub referenced_message: Option<Box<Message>>,
    pub thread: Option<Thread>,
    pub position: Option<i64>,
    pub application: Option<Application>,
    pub application_id: Option<String>,
    pub interaction: Option<Interaction>,
    pub activity_instance: Option<ActivityInstance>,
    pub interaction_metadata: Option<InteractionMetadata>,
    pub poll: Option<Poll>,
    pub sticker_items: Option<Vec<StickerItem>>,
    pub resolved: Option<Resolved>,
}

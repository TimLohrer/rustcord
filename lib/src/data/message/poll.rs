use serde::{Deserialize, Serialize};

use crate::data::channel::icon_emoji::IconEmoji;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Poll {
    pub question: PollQuestion,
    pub answers: Vec<PollAnswer>,
    pub expiry: String,
    pub allow_multiselect: bool,
    pub layout_type: i64,
    pub results: PollResults,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollQuestion {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollResults {
    pub answer_counts: Vec<PollAnswerCount>,
    pub is_finalized: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollAnswerCount {
    pub id: i64,
    pub count: i64,
    pub me_voted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollMedia {
    pub text: String,
    pub emoji: Option<IconEmoji>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollAnswer {
    pub answer_id: i64,
    pub poll_media: PollMedia,
}
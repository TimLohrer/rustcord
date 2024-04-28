use serde::{Deserialize, Serialize};

use crate::data::channel::icon_emoji::IconEmoji;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Poll {
    question: PollQuestion,
    answers: Vec<PollAnswer>,
    expiry: String,
    allow_multiselect: bool,
    layout_type: i64,
    results: PollResults,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollQuestion {
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollResults {
    answer_counts: Vec<PollAnswerCount>,
    is_finalized: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollAnswerCount {
    id: i64,
    count: i64,
    me_voted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollMedia {
    text: String,
    emoji: Option<IconEmoji>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollAnswer {
    answer_id: i64,
    poll_media: PollMedia,
}
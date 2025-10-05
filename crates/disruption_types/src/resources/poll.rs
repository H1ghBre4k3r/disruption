use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::entities::EmojiApiType;

/// <https://discord.com/developers/docs/resources/poll#poll-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollApiType {
    /// The question of the poll. Only text is supported.
    pub question: PollMediaApiType,
    /// Each of the answers available in the poll.
    pub answers: Vec<PollAnswerApiType>,
    /// The time when the poll ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// Whether a user can select multiple answers
    pub allow_multiselect: bool,
    /// The layout type of the poll
    pub layout_type: PollLayoutType,
    /// The results of the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<PollResultsApiType>,
}

/// <https://discord.com/developers/docs/resources/poll#poll-media-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollMediaApiType {
    /// The text of the field (max 300 characters for question, max 55 for answer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The emoji of the field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<EmojiApiType>,
}

/// <https://discord.com/developers/docs/resources/poll#poll-answer-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollAnswerApiType {
    /// The ID of the answer
    pub answer_id: u32,
    /// The data of the answer
    pub poll_media: PollMediaApiType,
}

/// <https://discord.com/developers/docs/resources/poll#poll-results-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollResultsApiType {
    /// Whether the votes have been precisely counted
    pub is_finalized: bool,
    /// The counts for each answer
    pub answer_counts: Vec<PollAnswerCountApiType>,
}

/// <https://discord.com/developers/docs/resources/poll#poll-answer-count-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollAnswerCountApiType {
    /// The answer_id
    pub id: u32,
    /// The number of votes for this answer
    pub count: u32,
    /// Whether the current user voted for this answer
    pub me_voted: bool,
}

/// <https://discord.com/developers/docs/resources/poll#layout-type>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PollLayoutType {
    /// The default layout type
    Default = 1,
}

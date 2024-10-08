use serde::{Deserialize, Serialize};

/**
 * `ChatMessage` struct - Represents a chat message and contains the data of each message a user sends
 */
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub uuid: Option<String>,
    pub user_id: String,
    pub text: String,
    pub message_sent_at: Option<String>,
}

impl ChatMessage {
    pub const fn new(uuid: String, user_id: String, text: String, message_sent_at: String) -> Self {
        Self {
            uuid: Some(uuid),
            user_id,
            text,
            message_sent_at: Some(message_sent_at),
        }
    }
}

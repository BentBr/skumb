use serde::{Deserialize, Serialize};

/**
 * ChatMessage struct - Represents a chat message and contains the data of each message a user sends
 */
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub uuid: Option<String>,
    pub user_uuid: String,
    pub text: String,
    pub message_sent_at: Option<String>,
}

impl ChatMessage {
    #[allow(dead_code)]
    pub const fn new(user_uuid: String, text: String) -> Self {
        Self {
            uuid: None,
            user_uuid,
            text,
            message_sent_at: None,
        }
    }
}

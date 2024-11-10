use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/**
 * `ChatMessage` struct - Represents a chat message and contains the data of each message a user sends
 */
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub uuid: Option<String>,
    pub user_id: String,
    pub cipher: String,
    pub iv: String,
    pub message_sent_at: Option<NaiveDateTime>,
}

impl ChatMessage {
    pub const fn new(
        uuid: String,
        user_id: String,
        cipher: String,
        iv: String,
        message_sent_at: NaiveDateTime,
    ) -> Self {
        Self {
            uuid: Some(uuid),
            user_id,
            cipher,
            iv,
            message_sent_at: Some(message_sent_at),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::json_serialization::web_socket::chat_message::ChatMessage;
    use chrono::NaiveDateTime;
    use serde_json;

    #[test]
    fn test_chat_message_serialize() {
        let chat_message = ChatMessage::new(
            "uuid123".to_string(),
            "user123".to_string(),
            "ciphertext".to_string(),
            "iv123".to_string(),
            NaiveDateTime::parse_from_str("2023-10-01T12:34:56", "%Y-%m-%dT%H:%M:%S").unwrap(),
        );
        let json = serde_json::to_string(&chat_message).unwrap();
        assert_eq!(
            json,
            r#"{"uuid":"uuid123","user_id":"user123","cipher":"ciphertext","iv":"iv123","message_sent_at":"2023-10-01T12:34:56"}"#
        );
    }

    #[test]
    fn test_chat_message_deserialize() {
        let json = r#"{"uuid":"uuid123","user_id":"user123","cipher":"ciphertext","iv":"iv123","message_sent_at":"2023-10-01T12:34:56"}"#;
        let chat_message: ChatMessage = serde_json::from_str(json).unwrap();
        assert_eq!(chat_message.uuid, Some("uuid123".to_string()));
        assert_eq!(chat_message.user_id, "user123".to_string());
        assert_eq!(chat_message.cipher, "ciphertext".to_string());
        assert_eq!(chat_message.iv, "iv123".to_string());
        assert_eq!(
            chat_message.message_sent_at,
            Some(NaiveDateTime::parse_from_str("2023-10-01T12:34:56", "%Y-%m-%dT%H:%M:%S").unwrap())
        );
    }

    #[test]
    fn test_chat_message_new() {
        let chat_message = ChatMessage::new(
            "uuid123".to_string(),
            "user123".to_string(),
            "ciphertext".to_string(),
            "iv123".to_string(),
            NaiveDateTime::parse_from_str("2023-10-01T12:34:56", "%Y-%m-%dT%H:%M:%S").unwrap(),
        );
        assert_eq!(chat_message.uuid, Some("uuid123".to_string()));
        assert_eq!(chat_message.user_id, "user123".to_string());
        assert_eq!(chat_message.cipher, "ciphertext".to_string());
        assert_eq!(chat_message.iv, "iv123".to_string());
        assert_eq!(
            chat_message.message_sent_at,
            Some(NaiveDateTime::parse_from_str("2023-10-01T12:34:56", "%Y-%m-%dT%H:%M:%S").unwrap())
        );
    }
}

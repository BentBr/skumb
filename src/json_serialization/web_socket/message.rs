use crate::json_serialization::web_socket::chat_message::ChatMessage;
use crate::json_serialization::web_socket::connection::Connection;
use crate::json_serialization::web_socket::ping::Ping;
use serde::{Deserialize, Serialize};

/**
 * Message struct - Represents a message that can be sent over the WebSocket.
 * Contains a message type and the data of the message.
 *
 ** `Pings` are used to determine if the connection is still active and the other sides still lives.
 ** `Connections` are used to change the connection status of the chat itself (active or inactive).
 ** `ChatMessages` are the actual messages being sent between users.
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Data {
    ChatMessage(ChatMessage),
    Connection(Connection),
    Ping(Ping),
}

impl Message {
    #[allow(dead_code)]
    pub const fn new(data: Data) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::super::chat_message::ChatMessage;
    use super::super::connection::Status::Connected;
    use super::super::ping::Ping;
    use super::Connection;
    use super::{Data, Message};
    use crate::json_serialization::web_socket::ping::Knock;

    #[test]
    fn test_data_enum_serialize() {
        let chat = ChatMessage {
            uuid: Some("123".to_string()),
            user_id: "user123".to_string(),
            text: "Hello".to_string(),
            message_sent_at: Some("2023-10-01T12:00:00Z".to_string()),
        };
        let data = Data::ChatMessage(chat);
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(
            json,
            r#"{"ChatMessage":{"uuid":"123","user_id":"user123","text":"Hello","message_sent_at":"2023-10-01T12:00:00Z"}}"#
        );

        let connection = Connection::new(Connected, "user123".to_string());
        let data = Data::Connection(connection);
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(json, r#"{"Connection":{"status":"Connected","user_id":"user123"}}"#);

        let ping = Ping::new(Knock::Ping);
        let data = Data::Ping(ping.clone());
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(json, r#"{"Ping":{"ping_type":"Ping"}}"#);
    }

    #[test]
    fn test_message_new() {
        let chat = ChatMessage {
            uuid: Some("123".to_string()),
            user_id: "user123".to_string(),
            text: "Hello".to_string(),
            message_sent_at: Some("2023-10-01T12:00:00Z".to_string()),
        };
        let message = Message::new(Data::ChatMessage(chat.clone()));

        assert_eq!(message.data, Data::ChatMessage(chat));
    }

    #[test]
    fn test_message_serialize() {
        let ping = Ping::new(Knock::Ping);
        let message = Message::new(Data::Ping(ping));
        let json = serde_json::to_string(&message).unwrap();

        assert_eq!(json, r#"{"data":{"Ping":{"ping_type":"Ping"}}}"#);
    }

    #[test]
    fn test_message_deserialize() {
        let json = r#"{"data":{"Ping":{"ping_type":"Ping"}}}"#;
        let message: Message = serde_json::from_str(json).unwrap();
        if let Data::Ping(ping) = message.data {
            assert_eq!(ping.ping_type, Knock::Ping);
        } else {
            panic!("Deserialization failed");
        }
    }
}

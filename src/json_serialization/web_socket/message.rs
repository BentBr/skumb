use crate::json_serialization::web_socket::chat_message::ChatMessage;
use crate::json_serialization::web_socket::connection::Connection;
use crate::json_serialization::web_socket::ping::Ping;
use serde::{Deserialize, Serialize};

/**
 * Message struct - Represents a message that can be sent over the WebSocket.
 * Contains a message type and the data of the message.
 *
 ** Pings are used to determine if the connection is still active and the other sides still lives.
 ** Connections are used to change the connection status of the chat itself (active or inactive).
 ** ChatMessages are the actual messages being sent between users.
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub message_type: Type,
    pub message_data: Data,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Type {
    ChatMessage,
    Connection,
    Ping,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Data {
    ChatMessage(ChatMessage),
    Connection(Connection),
    Ping(Ping),
}

impl Message {
    #[allow(dead_code)]
    pub const fn new(message_type: Type, message_data: Data) -> Self {
        Self {
            message_type,
            message_data,
        }
    }
}

impl Type {
    #[allow(dead_code)]
    pub fn stringify(&self) -> String {
        match self {
            Self::ChatMessage => "ChatMessage".to_string(),
            Self::Connection => "Connection".to_string(),
            Self::Ping => "Ping".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn from_string(input_string: &str) -> Self {
        match input_string {
            "ChatMessage" => Self::ChatMessage,
            "Connection" => Self::Connection,
            "Ping" => Self::Ping,
            _ => panic!("Input '{input_string}' not supported as at valid message type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::chat_message::ChatMessage;
    use super::super::connection::Status::Connected;
    use super::super::ping::Ping;
    use super::Connection;
    use super::{Data, Message, Type};
    use crate::json_serialization::web_socket::ping::Knock;

    #[test]
    fn test_type_enum_stringify() {
        assert_eq!(Type::ChatMessage.stringify(), "ChatMessage");
        assert_eq!(Type::Connection.stringify(), "Connection");
        assert_eq!(Type::Ping.stringify(), "Ping");
    }

    #[test]
    fn test_type_enum_from_string() {
        assert_eq!(Type::from_string("ChatMessage"), Type::ChatMessage);
        assert_eq!(Type::from_string("Connection"), Type::Connection);
        assert_eq!(Type::from_string("Ping"), Type::Ping);
    }

    #[test]
    #[should_panic]
    fn test_type_enum_from_string_panic() {
        Type::from_string("Invalid");
    }

    #[test]
    fn test_data_enum_serialize() {
        let chat = ChatMessage {
            uuid: Some("123".to_string()),
            user_uuid: "user123".to_string(),
            text: "Hello".to_string(),
            message_sent_at: Some("2023-10-01T12:00:00Z".to_string()),
        };
        let data = Data::ChatMessage(chat);
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(
            json,
            r#"{"ChatMessage":{"uuid":"123","user_uuid":"user123","text":"Hello","message_sent_at":"2023-10-01T12:00:00Z"}}"#
        );

        let connection = Connection::new(Connected);
        let data = Data::Connection(connection);
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(json, r#"{"Connection":{"status":"Connected"}}"#);

        let ping = Ping::new(Knock::Ping);
        let data = Data::Ping(ping.clone());
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(json, r#"{"Ping":{"ping_type":"Ping"}}"#);
    }

    #[test]
    fn test_message_new() {
        let chat = ChatMessage {
            uuid: Some("123".to_string()),
            user_uuid: "user123".to_string(),
            text: "Hello".to_string(),
            message_sent_at: Some("2023-10-01T12:00:00Z".to_string()),
        };
        let message = Message::new(Type::ChatMessage, Data::ChatMessage(chat.clone()));

        assert_eq!(message.message_type, Type::ChatMessage);
        assert_eq!(message.message_data, Data::ChatMessage(chat));
    }

    #[test]
    fn test_message_serialize() {
        let ping = Ping::new(Knock::Ping);
        let message = Message::new(Type::Ping, Data::Ping(ping));
        let json = serde_json::to_string(&message).unwrap();

        assert_eq!(
            json,
            r#"{"message_type":"Ping","message_data":{"Ping":{"ping_type":"Ping"}}}"#
        );
    }

    #[test]
    fn test_message_deserialize() {
        let json = r#"{"message_type":"Ping","message_data":{"Ping":{"ping_type":"Ping"}}}"#;
        let message: Message = serde_json::from_str(json).unwrap();
        if let Data::Ping(ping) = message.message_data {
            assert_eq!(ping.ping_type, Knock::Ping);
        } else {
            panic!("Deserialization failed");
        }
    }
}

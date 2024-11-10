use crate::json_serialization::web_socket::chat_message::ChatMessage;
use crate::json_serialization::web_socket::connection::Connection;
use crate::json_serialization::web_socket::group_key::GroupKey;
use crate::json_serialization::web_socket::ping::Ping;
use serde::{Deserialize, Serialize};

/**
 * Message struct - Represents a message that can be sent over the WebSocket.
 * Contains a message type and the data of the message.
 *
 ** `Pings` are used to determine if the connection is still active and the other sides still lives.
 ** `Connections` are used to change the connection status of the chat itself (active or inactive) via the participants
 ** sending their status
 ** `ChatMessages` are the actual messages being sent between users.
 ** `GroupKeys` are used to send the current group key to all participants but encrypted for one specific user.
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Data {
    ChatMessage(ChatMessage),
    Connection(Connection),
    GroupKey(GroupKey),
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
    use crate::json_serialization::web_socket::connection::PublicKey;
    use crate::json_serialization::web_socket::group_key::GroupKey;
    use crate::json_serialization::web_socket::ping::Knock;
    use chrono::NaiveDateTime;

    #[test]
    fn test_message_new() {
        let chat = ChatMessage {
            uuid: Some("123".to_string()),
            user_id: "user123".to_string(),
            cipher: "Hello encrypted cipher".to_string(),
            iv: "iv".to_string(),
            message_sent_at: Some(NaiveDateTime::parse_from_str("2023-10-01T12:34:56", "%Y-%m-%dT%H:%M:%S").unwrap()),
        };
        let message = Message::new(Data::ChatMessage(chat.clone()));

        assert_eq!(message.data, Data::ChatMessage(chat));
    }

    #[test]
    fn test_data_enum_serialize() {
        let chat = ChatMessage {
            uuid: Some("123".to_string()),
            user_id: "user123".to_string(),
            cipher: "Hello encrypted cipher".to_string(),
            iv: "iv".to_string(),
            message_sent_at: Some(NaiveDateTime::parse_from_str("2023-10-01T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()),
        };

        let data = Data::ChatMessage(chat);
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(
            json,
            r#"{"ChatMessage":{"uuid":"123","user_id":"user123","cipher":"Hello encrypted cipher","iv":"iv","message_sent_at":"2023-10-01T12:00:00"}}"#
        );

        let public_key = PublicKey {
            crv: "P-384".to_string(),
            ext: true,
            key_ops: vec![],
            kty: "EC".to_string(),
            x: "Br-DF2-zNbZUrIbRcmiHw-b5QjWpOuii1KzgYQRqXvFtQrzXf410i4ir6lPBmpW0".to_string(),
            y: "_2-ErGT-IwIg-K3TQgLkeMLfbw-CQxpmGLDGgykRxpHgfnFwENRbmkDWqPPQPHgC".to_string(),
        };
        let connection = Connection::new(Connected, "user123".to_string(), "user name".to_string(), public_key);
        let data = Data::Connection(connection);
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(
            json,
            r#"{"Connection":{"status":"Connected","user_id":"user123","user_name":"user name","public_key":{"crv":"P-384","ext":true,"key_ops":[],"kty":"EC","x":"Br-DF2-zNbZUrIbRcmiHw-b5QjWpOuii1KzgYQRqXvFtQrzXf410i4ir6lPBmpW0","y":"_2-ErGT-IwIg-K3TQgLkeMLfbw-CQxpmGLDGgykRxpHgfnFwENRbmkDWqPPQPHgC"}}}"#
        );

        let ping = Ping::new(Knock::Ping);
        let data = Data::Ping(ping.clone());
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(json, r#"{"Ping":{"ping_type":"Ping"}}"#);

        let time = NaiveDateTime::parse_from_str("2023-10-01T12:34:56", "%Y-%m-%dT%H:%M:%S").unwrap();
        let group_key = GroupKey {
            encrypted_key: "some_key".to_string(),
            iv: "iv".to_string(),
            creation_date: time.clone(),
            for_user_id: "user123".to_string(),
            from_user_id: "user=6789".to_string(),
        };
        let data = Data::GroupKey(group_key);
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(
            json,
            r#"{"GroupKey":{"encrypted_key":"some_key","iv":"iv","creation_date":"2023-10-01T12:34:56","for_user_id":"user123","from_user_id":"user=6789"}}"#
        );
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

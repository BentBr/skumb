use serde::{Deserialize, Serialize};

/**
 * Connection struct - Represents the connection status of the WebSocket
 * (true if connected, false if disconnected)
 */
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Connection {
    pub status: Status,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Connected,
    StayingAlive,
    Disconnected,
}

impl Connection {
    #[allow(dead_code)]
    pub const fn new(status: Status, user_id: String) -> Self {
        Self { status, user_id }
    }
}

#[cfg(test)]
mod tests {
    use super::Connection;
    use super::Status::{Connected, StayingAlive, Disconnected};

    #[test]
    fn test_connection_new() {
        let connection = Connection::new(Connected, "user123".to_string());
        assert_eq!(connection.status, Connected);
        assert_eq!(connection.user_id, "user123".to_string());

        let connection = Connection::new(StayingAlive, "user123".to_string());
        assert_eq!(connection.status, StayingAlive);
    }

    #[test]
    fn test_connection_serialize() {
        let connection = Connection::new(Connected, "user123".to_string());
        let json = serde_json::to_string(&connection).unwrap();
        assert_eq!(json, r#"{"status":"Connected","user_id":"user123"}"#);

        let connection = Connection::new(Disconnected, "user123".to_string());
        let json = serde_json::to_string(&connection).unwrap();
        assert_eq!(json, r#"{"status":"Disconnected","user_id":"user123"}"#);

        let connection = Connection::new(StayingAlive, "user123".to_string());
        let json = serde_json::to_string(&connection).unwrap();
        assert_eq!(json, r#"{"status":"StayingAlive","user_id":"user123"}"#);
    }

    #[test]
    fn test_connection_deserialize() {
        let json = r#"{"status":"Connected","user_id":"user123"}"#;
        let connection: Connection = serde_json::from_str(json).unwrap();
        assert_eq!(connection.status, Connected);
        assert_eq!(connection.user_id, "user123".to_string());

        let json = r#"{"status":"Disconnected","user_id":"user123"}"#;
        let connection: Connection = serde_json::from_str(json).unwrap();
        assert_eq!(connection.status, Disconnected);
        assert_eq!(connection.user_id, "user123".to_string());

        let json = r#"{"status":"StayingAlive","user_id":"user123"}"#;
        let connection: Connection = serde_json::from_str(json).unwrap();
        assert_eq!(connection.status, StayingAlive);
        assert_eq!(connection.user_id, "user123".to_string());
    }

    #[test]
    #[should_panic]
    fn test_connection_from_string_panic() {
        serde_json::from_str::<Connection>("Invalid").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_connection_from_string_panic_no_user_id() {
        serde_json::from_str::<Connection>(r#"{"status":"Disconnected"}"#).unwrap();
    }
}

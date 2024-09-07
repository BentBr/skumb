use serde::{Deserialize, Serialize};

/**
 * Connection struct - Represents the connection status of the WebSocket
 * (true if connected, false if disconnected)
 */
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Connection {
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Connected,
    Disconnected,
}

impl Connection {
    #[allow(dead_code)]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }
}

#[cfg(test)]
mod tests {
    use super::Connection;
    use super::Status::{Connected, Disconnected};

    #[test]
    fn test_connection_new() {
        let connection = Connection::new(Connected);
        assert_eq!(connection.status, Connected);
    }

    #[test]
    fn test_connection_serialize() {
        let connection = Connection::new(Connected);
        let json = serde_json::to_string(&connection).unwrap();
        assert_eq!(json, r#"{"status":"Connected"}"#);

        let connection = Connection::new(Disconnected);
        let json = serde_json::to_string(&connection).unwrap();
        assert_eq!(json, r#"{"status":"Disconnected"}"#);
    }

    #[test]
    fn test_connection_deserialize() {
        let json = r#"{"status":"Connected"}"#;
        let connection: Connection = serde_json::from_str(json).unwrap();
        assert_eq!(connection.status, Connected);

        let json = r#"{"status":"Disconnected"}"#;
        let connection: Connection = serde_json::from_str(json).unwrap();
        assert_eq!(connection.status, Disconnected);
    }

    #[test]
    #[should_panic]
    fn test_connection_from_string_panic() {
        serde_json::from_str::<Connection>("Invalid").unwrap();
    }
}

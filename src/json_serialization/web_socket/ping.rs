use serde::{Deserialize, Serialize};

/**
 * Ping struct - Represents a ping message sent to the WebSocket server
 * Used to determine if the other side is active.
 */
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Ping {
    pub ping_type: Knock,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Knock {
    Ping,
    Pong,
}

impl Ping {
    #[allow(dead_code)]
    pub const fn new(ping_type: Knock) -> Self {
        Self { ping_type }
    }
}

#[cfg(test)]
mod tests {
    use super::{Knock, Ping};

    #[test]
    fn test_ping_new() {
        let ping = Ping::new(Knock::Ping);
        assert_eq!(ping.ping_type, Knock::Ping);
    }

    #[test]
    fn test_ping_serialize() {
        let ping = Ping { ping_type: Knock::Ping };
        let json = serde_json::to_string(&ping).unwrap();
        assert_eq!(json, r#"{"ping_type":"Ping"}"#);

        let pong = Ping { ping_type: Knock::Pong };
        let json = serde_json::to_string(&pong).unwrap();
        assert_eq!(json, r#"{"ping_type":"Pong"}"#);
    }

    #[test]
    fn test_ping_deserialize() {
        let json = r#"{"ping_type":"Ping"}"#;
        let ping: Ping = serde_json::from_str(json).unwrap();
        assert_eq!(ping.ping_type, Knock::Ping);

        let json = r#"{"ping_type":"Pong"}"#;
        let pong: Ping = serde_json::from_str(json).unwrap();
        assert_eq!(pong.ping_type, Knock::Pong);
    }

    #[test]
    fn test_knock_serialize() {
        assert_eq!(serde_json::to_string(&Knock::Ping).unwrap(), r#""Ping""#);
        assert_eq!(serde_json::to_string(&Knock::Pong).unwrap(), r#""Pong""#);
    }

    #[test]
    fn test_knock_deserialize() {
        assert_eq!(serde_json::from_str::<Knock>(r#""Ping""#).unwrap(), Knock::Ping);
        assert_eq!(serde_json::from_str::<Knock>(r#""Pong""#).unwrap(), Knock::Pong);
    }

    #[test]
    #[should_panic]
    fn test_ping_deserialize_panic() {
        serde_json::from_str::<Ping>("Invalid").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_knock_deserialize_panic() {
        serde_json::from_str::<Knock>("Invalid").unwrap();
    }
}

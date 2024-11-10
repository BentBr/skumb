use serde::{Deserialize, Serialize};

/**
 * `Connection` struct - Represents the connection status of the WebSocket
 * Contains all needed user data.
 */
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Connection {
    pub status: Status,
    pub user_id: String,
    pub user_name: String,
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Connected,
    StayingAlive,
    Disconnected,
}

/**
* `PublicKey` struct - Represents the public key of the user when connecting.
* Created with the crypto.subtle API in JavaScript and exported as JWK.
*/
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    pub crv: String,
    pub ext: bool,
    pub key_ops: Vec<String>,
    pub kty: String,
    pub x: String,
    pub y: String,
}

impl Connection {
    #[allow(dead_code)]
    pub const fn new(status: Status, user_id: String, user_name: String, public_key: PublicKey) -> Self {
        Self {
            status,
            user_id,
            user_name,
            public_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Status::{Connected, Disconnected, StayingAlive};
    use super::{Connection, PublicKey};

    #[test]
    fn test_connection_new() {
        let public_key = PublicKey {
            crv: "P-384".to_string(),
            ext: true,
            key_ops: vec![],
            kty: "EC".to_string(),
            x: "Br-DF2-zNbZUrIbRcmiHw-b5QjWpOuii1KzgYQRqXvFtQrzXf410i4ir6lPBmpW0".to_string(),
            y: "_2-ErGT-IwIg-K3TQgLkeMLfbw-CQxpmGLDGgykRxpHgfnFwENRbmkDWqPPQPHgC".to_string(),
        };
        let connection = Connection::new(
            Connected,
            "user123".to_string(),
            "user name".to_string(),
            public_key.clone(),
        );
        assert_eq!(connection.status, Connected);
        assert_eq!(connection.user_id, "user123".to_string());
        assert_eq!(connection.user_name, "user name".to_string());
        assert_eq!(connection.public_key, public_key);

        let connection = Connection::new(StayingAlive, "user123".to_string(), "user name".to_string(), public_key);
        assert_eq!(connection.status, StayingAlive);
    }

    #[test]
    fn test_connection_serialize() {
        let public_key = PublicKey {
            crv: "P-384".to_string(),
            ext: true,
            key_ops: vec![],
            kty: "EC".to_string(),
            x: "Br-DF2-zNbZUrIbRcmiHw-b5QjWpOuii1KzgYQRqXvFtQrzXf410i4ir6lPBmpW0".to_string(),
            y: "_2-ErGT-IwIg-K3TQgLkeMLfbw-CQxpmGLDGgykRxpHgfnFwENRbmkDWqPPQPHgC".to_string(),
        };
        let connection = Connection::new(
            Connected,
            "user123".to_string(),
            "user name".to_string(),
            public_key.clone(),
        );
        let json = serde_json::to_string(&connection).unwrap();
        assert_eq!(
            json,
            r#"{"status":"Connected","user_id":"user123","user_name":"user name","public_key":{"crv":"P-384","ext":true,"key_ops":[],"kty":"EC","x":"Br-DF2-zNbZUrIbRcmiHw-b5QjWpOuii1KzgYQRqXvFtQrzXf410i4ir6lPBmpW0","y":"_2-ErGT-IwIg-K3TQgLkeMLfbw-CQxpmGLDGgykRxpHgfnFwENRbmkDWqPPQPHgC"}}"#
        );

        let connection = Connection::new(
            Disconnected,
            "user123".to_string(),
            "user name".to_string(),
            public_key.clone(),
        );
        let json = serde_json::to_string(&connection).unwrap();
        assert_eq!(
            json,
            r#"{"status":"Disconnected","user_id":"user123","user_name":"user name","public_key":{"crv":"P-384","ext":true,"key_ops":[],"kty":"EC","x":"Br-DF2-zNbZUrIbRcmiHw-b5QjWpOuii1KzgYQRqXvFtQrzXf410i4ir6lPBmpW0","y":"_2-ErGT-IwIg-K3TQgLkeMLfbw-CQxpmGLDGgykRxpHgfnFwENRbmkDWqPPQPHgC"}}"#
        );

        let connection = Connection::new(StayingAlive, "user123".to_string(), "user name".to_string(), public_key);
        let json = serde_json::to_string(&connection).unwrap();
        assert_eq!(
            json,
            r#"{"status":"StayingAlive","user_id":"user123","user_name":"user name","public_key":{"crv":"P-384","ext":true,"key_ops":[],"kty":"EC","x":"Br-DF2-zNbZUrIbRcmiHw-b5QjWpOuii1KzgYQRqXvFtQrzXf410i4ir6lPBmpW0","y":"_2-ErGT-IwIg-K3TQgLkeMLfbw-CQxpmGLDGgykRxpHgfnFwENRbmkDWqPPQPHgC"}}"#
        );
    }

    #[test]
    fn test_connection_deserialize() {
        let json = r#"{"status":"Connected","user_id":"user123","user_name":"user name","public_key":{"crv":"P-384","ext":true,"key_ops":[],"kty":"EC","x":"Br-DF2-zNbZUrIbRcmiHw-b5QjWpOuii1KzgYQRqXvFtQrzXf410i4ir6lPBmpW0","y":"_2-ErGT-IwIg-K3TQgLkeMLfbw-CQxpmGLDGgykRxpHgfnFwENRbmkDWqPPQPHgC"}}"#;
        let public_key = PublicKey {
            crv: "P-384".to_string(),
            ext: true,
            key_ops: vec![],
            kty: "EC".to_string(),
            x: "Br-DF2-zNbZUrIbRcmiHw-b5QjWpOuii1KzgYQRqXvFtQrzXf410i4ir6lPBmpW0".to_string(),
            y: "_2-ErGT-IwIg-K3TQgLkeMLfbw-CQxpmGLDGgykRxpHgfnFwENRbmkDWqPPQPHgC".to_string(),
        };
        let connection: Connection = serde_json::from_str(json).unwrap();
        assert_eq!(connection.status, Connected);
        assert_eq!(connection.user_id, "user123".to_string());
        assert_eq!(connection.user_name, "user name".to_string());
        assert_eq!(connection.public_key, public_key.clone());

        let json = r#"{"status":"Disconnected","user_id":"user123","user_name":"user name","public_key":{"crv": "P-384","ext": true,"key_ops": [],"kty": "EC","x": "Br-DF2-zNbZUrIbRcmiHw-b5QjWpOuii1KzgYQRqXvFtQrzXf410i4ir6lPBmpW0","y": "_2-ErGT-IwIg-K3TQgLkeMLfbw-CQxpmGLDGgykRxpHgfnFwENRbmkDWqPPQPHgC"}}"#;
        let connection: Connection = serde_json::from_str(json).unwrap();
        assert_eq!(connection.status, Disconnected);
        assert_eq!(connection.user_id, "user123".to_string());
        assert_eq!(connection.user_name, "user name".to_string());
        assert_eq!(connection.public_key, public_key.clone());

        let json = r#"{"status":"StayingAlive","user_id":"user123","user_name":"user name","public_key":{"crv": "P-384","ext": true,"key_ops": [],"kty": "EC","x": "Br-DF2-zNbZUrIbRcmiHw-b5QjWpOuii1KzgYQRqXvFtQrzXf410i4ir6lPBmpW0","y": "_2-ErGT-IwIg-K3TQgLkeMLfbw-CQxpmGLDGgykRxpHgfnFwENRbmkDWqPPQPHgC"}}"#;
        let connection: Connection = serde_json::from_str(json).unwrap();
        assert_eq!(connection.status, StayingAlive);
        assert_eq!(connection.user_id, "user123".to_string());
        assert_eq!(connection.user_name, "user name".to_string());
        assert_eq!(connection.public_key, public_key);
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

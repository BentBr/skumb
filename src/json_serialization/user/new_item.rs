use crate::models::user::item::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NewItem {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl NewItem {
    pub fn _new(input_item: User) -> Self {
        Self {
            username: input_item.username,
            email: input_item.email,
            password: input_item.password,
        }
    }
}

#[cfg(test)]
mod new_user_item_tests {
    use super::NewItem;
    use crate::models::user::item::User;

    #[test]
    fn new() {
        let user = User {
            id: 0,
            uuid: Default::default(),
            username: "john doe".to_string(),
            salutation: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "john@example.com".to_string(),
            password: "password123".to_string(),
            creation_date: Default::default(),
            modification_date: None,
            deletion_date: None,
        };

        let new_user_item = NewItem::_new(user);

        assert_eq!(new_user_item.username, "john doe");
        assert_eq!(new_user_item.email, "john@example.com");
        assert_eq!(new_user_item.password, "password123");
    }

    #[test]
    fn serialize() {
        let new_user_item = NewItem {
            username: "john doe".to_string(),
            email: "john@example.com".to_string(),
            password: "password123".to_string(),
        };

        let serialized = serde_json::to_string(&new_user_item).unwrap();
        let expected = r#"{"username":"john doe","email":"john@example.com","password":"password123"}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn deserialize() {
        let json = r#"{"username":"john doe","email":"john@example.com","password":"password123"}"#;
        let deserialized: NewItem = serde_json::from_str(json).unwrap();

        assert_eq!(deserialized.username, "john doe");
        assert_eq!(deserialized.email, "john@example.com");
        assert_eq!(deserialized.password, "password123");
    }
}

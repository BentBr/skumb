use crate::json_serialization::user::item::Item;
use crate::models::user::item::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct Items {
    pub user_items: Vec<Item>,
    pub user_items_count: usize,
}

impl Items {
    pub fn new(input_items: Vec<User>) -> Self {
        let mut user_array_buffer = Vec::new();

        for item in input_items {
            let user_item = Item::new(&item);

            user_array_buffer.push(user_item);
        }

        let open_count = user_array_buffer.len();

        Self {
            user_items: user_array_buffer,
            user_items_count: open_count,
        }
    }
}

#[cfg(test)]
mod user_items_tests {
    use super::Items;
    use crate::helpers::datetime::format;
    use crate::models::user::item::User;
    use chrono::{NaiveDateTime, Utc};
    use uuid::Uuid;

    fn create_sample_user(uuid: Uuid, username: &str, email: &str, time: NaiveDateTime) -> User {
        User {
            id: 0,
            uuid,
            username: username.to_string(),
            email: email.to_string(),
            password: "".to_string(),
            salutation: "Mr.".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            creation_date: time,
            modification_date: Some(time),
            deletion_date: None,
        }
    }

    #[test]
    fn new() {
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        let time = Utc::now().naive_utc();

        let users = vec![
            create_sample_user(uuid1, "user1", "user1@example.com", time.clone()),
            create_sample_user(uuid2, "user2", "user2@example.com", time.clone()),
        ];

        let user_items = Items::new(users.clone());

        assert_eq!(user_items.user_items.len(), 2);
        assert_eq!(user_items.user_items_count, 2);

        assert_eq!(user_items.user_items[0].uuid, uuid1.to_string());
        assert_eq!(user_items.user_items[0].username, "user1");
        assert_eq!(user_items.user_items[0].email, "user1@example.com");
        assert_eq!(user_items.user_items[0].salutation, "Mr.");
        assert_eq!(user_items.user_items[0].first_name, "John");
        assert_eq!(user_items.user_items[0].last_name, "Doe");
        assert_eq!(user_items.user_items[0].creation_date, time.to_string());
        assert_eq!(
            user_items.user_items[0].modification_date,
            format(users[0].modification_date)
        );
        assert_eq!(user_items.user_items[0].deletion_date, format(users[0].deletion_date));

        assert_eq!(user_items.user_items[1].uuid, uuid2.to_string());
        assert_eq!(user_items.user_items[1].username, "user2");
        assert_eq!(user_items.user_items[1].email, "user2@example.com");
        assert_eq!(user_items.user_items[1].salutation, "Mr.");
        assert_eq!(user_items.user_items[1].first_name, "John");
        assert_eq!(user_items.user_items[1].last_name, "Doe");
        assert_eq!(user_items.user_items[1].creation_date, time.to_string());
        assert_eq!(
            user_items.user_items[1].modification_date,
            format(users[1].modification_date)
        );
        assert_eq!(user_items.user_items[1].deletion_date, format(users[1].deletion_date));
    }

    #[test]
    fn serialize() {
        let uuid_string1 = "72655de0-21e6-40f0-9856-9530344bf78d";
        let uuid_string2 = "85979ec6-66c5-4ba4-9153-606f2e9e2f6a";
        let time = NaiveDateTime::parse_from_str("2022-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let users = vec![
            create_sample_user(
                Uuid::parse_str(uuid_string1).unwrap(),
                "user1",
                "user1@example.com",
                time.clone(),
            ),
            create_sample_user(
                Uuid::parse_str(uuid_string2).unwrap(),
                "user2",
                "user2@example.com",
                time.clone(),
            ),
        ];

        let user_items = Items::new(users);

        let serialized = serde_json::to_string(&user_items).unwrap();
        let expected = r#"{"user_items":[{"uuid":"72655de0-21e6-40f0-9856-9530344bf78d","username":"user1","salutation":"Mr.","first_name":"John","last_name":"Doe","email":"user1@example.com","creation_date":"2022-01-01 00:00:00","modification_date":"2022-01-01 00:00:00","deletion_date":null},{"uuid":"85979ec6-66c5-4ba4-9153-606f2e9e2f6a","username":"user2","salutation":"Mr.","first_name":"John","last_name":"Doe","email":"user2@example.com","creation_date":"2022-01-01 00:00:00","modification_date":"2022-01-01 00:00:00","deletion_date":null}],"user_items_count":2}"#;

        assert_eq!(serialized, expected);
    }
}

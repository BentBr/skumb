use crate::json_serialization::response::status::Status;
use serde::Serialize;

#[derive(Serialize)]
pub struct Item<T>
where
    T: Serialize,
{
    pub status: Status,
    pub message: String,
    pub data: T,
}

impl<T> Item<T>
where
    T: Serialize,
{
    pub const fn new(status: Status, message: String, data: T) -> Self {
        Self { status, message, data }
    }
}

#[cfg(test)]
mod response_item_tests {
    use super::Item;
    use crate::json_serialization::response::status::Status;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestData {
        value: i32,
    }

    #[test]
    fn new() {
        let test_data = TestData { value: 42 };
        let response_item = Item::new(Status::Success, "Test message".to_string(), test_data);

        assert_eq!(response_item.status, Status::Success);
        assert_eq!(response_item.message, "Test message");
        assert_eq!(response_item.data.value, 42);
    }

    #[test]
    fn serialize() {
        let test_data = TestData { value: 42 };
        let response_item = Item::new(Status::Success, "Test message".to_string(), test_data);

        let serialized = serde_json::to_string(&response_item).unwrap();
        let expected = r#"{"status":"Success","message":"Test message","data":{"value":42}}"#;

        assert_eq!(serialized, expected);
    }
}

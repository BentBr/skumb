use lambda_http::{Body, Error, Response};
use serde::Serialize;
use serde_json::json;

pub struct JsonResponse<T>
where
    T: Serialize,
{
    pub status_code: u16,
    pub json_message: T,
}

impl<T> JsonResponse<T>
where
    T: Serialize,
{
    pub fn new(status_code: u16, json_message: T) -> Result<Response<Body>, Error> {
        Response::builder()
            .status(status_code)
            .header("content-type", "application/json")
            .body(json!(json_message).to_string().into())
            .map_err(Box::new)
    }
}

#[cfg(test)]
mod json_response_tests {
    use crate::json_serialization::response::json_response::JsonResponse;
    use lambda_http::Body;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestData {
        status: i32,
        message: String,
    }

    #[test]
    fn new() {
        let test_data = TestData {
            status: 234234,
            message: "Test message".to_string(),
        };

        let response_item = JsonResponse::new(422, test_data);

        assert_eq!(response_item.status(), 422);
        assert_eq!(
            response_item.into_body(),
            Body::from("{\"status\":234234,\"message\":\"Test message\"}")
        );
    }
}

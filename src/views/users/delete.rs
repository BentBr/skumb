use crate::database::DB;
use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::jwt::JwToken;
use crate::models::user::item::delete_item;
use actix_web::{HttpRequest};
use lambda_http::{Body, Response};
use uuid::Uuid;
use crate::json_serialization::response::json_response::JsonResponse;

pub async fn delete(request: HttpRequest, db: DB, _: JwToken) -> Response<Body> {
    let uuid: Uuid = match parse_uuid_from_request(request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    match delete_item(uuid, db) {
        Some(uuid) => JsonResponse::new(
            200,
            ResponseItem::new(
                ResponseStatus::Success,
                "Deleted user".to_string(),
                format!("Done with success: {}", uuid),
            ))?,
        None => JsonResponse::new(404, ResponseItem::new(
            ResponseStatus::Error,
            "Could not delete".to_string(),
            "Not found",
        ))?
    }
}

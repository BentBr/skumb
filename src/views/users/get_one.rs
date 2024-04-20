use crate::database::DB;
use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::user::user_item::UserItem;
use crate::jwt::JwToken;
use crate::models::user::item::fetch_item;
use actix_web::{HttpRequest, HttpResponse};
use lambda_http::{Body, Response};
use uuid::Uuid;
use crate::json_serialization::response::json_response::JsonResponse;

pub async fn get_one(request: HttpRequest, db: DB, _: JwToken) -> Response<Body> {
    let uuid: Uuid = match parse_uuid_from_request(request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    // Loading it
    let item = fetch_item(uuid, db);

    match item.first() {
        Some(item) => JsonResponse::new(200, ResponseItem::new(
            ResponseStatus::Success,
            "Fetched one user".to_string(),
            UserItem::new(item.clone()),
        ))?,
        None => JsonResponse::new(404, ResponseItem::new(
            ResponseStatus::Error,
            "Error during user lookup".to_string(),
            "Could not find it",
        ))?,
    }
}

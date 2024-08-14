use crate::database::DB;
use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::response::item::Item as ResponseItem;
use crate::json_serialization::response::status::Status;
use crate::json_serialization::user::item::Item as UserItem;
use crate::jwt::JwToken;
use crate::models::user::item::fetch;
use actix_web::{HttpRequest, HttpResponse};
use uuid::Uuid;

#[allow(clippy::future_not_send)]
pub async fn get_one(request: HttpRequest, db: DB, _: JwToken) -> HttpResponse {
    let uuid: Uuid = match parse_uuid_from_request(&request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    // Loading it
    let item = fetch(uuid, db);

    item.first().map_or_else(
        || {
            HttpResponse::NotFound().json(ResponseItem::new(
                Status::Error,
                "Error during user lookup".to_string(),
                "Could not find it",
            ))
        },
        |item| {
            HttpResponse::Ok().json(ResponseItem::new(
                Status::Success,
                "Fetched one user".to_string(),
                UserItem::new(item),
            ))
        },
    )
}

use crate::database::DB;
use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::response::item::Item;
use crate::json_serialization::response::status::Status;
use crate::jwt::JwToken;
use crate::models::user::item::delete as delete_item;
use actix_web::{HttpRequest, HttpResponse};
use uuid::Uuid;

#[allow(clippy::future_not_send)]
pub async fn delete(request: HttpRequest, db: DB, _: JwToken) -> HttpResponse {
    let uuid: Uuid = match parse_uuid_from_request(&request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    delete_item(uuid, db).map_or_else(
        || {
            HttpResponse::NotFound().json(Item::new(
                Status::Error,
                "Could not delete".to_string(),
                "Not found",
            ))
        },
        |uuid| {
            HttpResponse::Ok().json(Item::new(
                Status::Success,
                "Deleted user".to_string(),
                format!("Done with success: {uuid}"),
            ))
        },
    )
}

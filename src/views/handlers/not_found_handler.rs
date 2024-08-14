use crate::json_serialization::response::item::Item;
use crate::json_serialization::response::status::Status;
use actix_web::HttpResponse;

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(Item::new(
        Status::Error,
        "Not found".to_string(),
        "Requested resource not found",
    ))
}

use crate::json_serialization::response::item::Item;
use crate::json_serialization::response::status::Status;
use actix_web::HttpResponse;
use uuid::Uuid;

pub async fn uuid() -> HttpResponse {
    let uuid = Uuid::new_v4();

    // Currently it's the same as for chat, but it's a different module
    HttpResponse::Created().json(Item::new(Status::Success, "UUID created".to_string(), uuid))
}

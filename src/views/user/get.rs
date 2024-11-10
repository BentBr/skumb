use crate::database::DB;
use crate::json_serialization::response::item::Item;
use crate::json_serialization::response::status::Status;
use crate::json_serialization::user::items::Items;
use crate::jwt::JwToken;
use crate::models::user::items::fetch;
use actix_web::HttpResponse;

pub async fn get(db: DB, _: JwToken) -> HttpResponse {
    // Loading them with default limit: 100
    let items = fetch(None, db);

    HttpResponse::Ok().json(Item::new(
        Status::Success,
        format!("Fetched {} user items", items.len()),
        Items::new(items),
    ))
}

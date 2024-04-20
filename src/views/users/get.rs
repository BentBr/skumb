use crate::database::DB;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::user::user_items::UserItems;
use crate::jwt::JwToken;
use crate::models::user::items::fetch_items;
use actix_web::HttpResponse;
use lambda_http::{Body, Response};
use crate::json_serialization::response::json_response::JsonResponse;

pub async fn get(db: DB, _: JwToken) -> Response<Body> {
    // Loading them with default limit: 100
    let items = fetch_items(None, db);

    JsonResponse::new(200, ResponseItem::new(
        ResponseStatus::Success,
        format!("Fetched {} user items", items.len()),
        UserItems::new(items),
    ))?
}

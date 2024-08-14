use crate::database::DB;
use crate::helpers::email::parse_email_from_string;
use crate::json_serialization::response::item::Item as ResponseItem;
use crate::json_serialization::response::status::Status;
use crate::json_serialization::user::item::Item as UserItem;
use crate::json_serialization::user::new_item::NewItem;
use crate::models::user::new_item::create_item;
use actix_web::{web, HttpResponse};
use sentry::Level;

#[allow(clippy::future_not_send)]
pub async fn create(new_user_item: web::Json<NewItem>, db: DB) -> HttpResponse {
    let username = String::from(&new_user_item.username);
    let email = String::from(&new_user_item.email);
    let password = String::from(&new_user_item.password);

    let valid_email: String = match parse_email_from_string(email) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    if password.is_empty() {
        return HttpResponse::UnprocessableEntity().json(ResponseItem::new(
            Status::Error,
            "Password constraint".to_string(),
            "Must not be empty",
        ));
    }

    // Creating in DB
    let item = create_item(username, valid_email, &password, db);

    item.first().map_or_else(
        || {
            // Logging a bit
            sentry::capture_message("Storing and lookup of new item failed!", Level::Error);

            HttpResponse::Conflict().json(ResponseItem::new(
                Status::Error,
                "Error during user lookup and creation".to_string(),
                new_user_item,
            ))
        },
        |item| {
            HttpResponse::Created().json(ResponseItem::new(
                Status::Success,
                "Created new user".to_string(),
                UserItem::new(item),
            ))
        },
    )
}

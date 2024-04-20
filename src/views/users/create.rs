use crate::database::DB;
use crate::helpers::email::parse_email_from_string;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::user::new_user_item::NewUserItem;
use crate::json_serialization::user::user_item::UserItem;
use crate::models::user::new_item::create_item;
use actix_web::{web, HttpResponse};
use sentry::Level;

pub async fn create(new_user_item: web::Json<NewUserItem>, db: DB) -> HttpResponse {
    let username = String::from(&new_user_item.username);
    let email = String::from(&new_user_item.email);
    let password = String::from(&new_user_item.password);

    let valid_email: String = match parse_email_from_string(email) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    if password.is_empty() {
        return HttpResponse::UnprocessableEntity().json(ResponseItem::new(
            ResponseStatus::Error,
            "Password constraint".to_string(),
            "Must not be empty",
        ));
    }

    // Creating in DB
    let item = create_item(username, valid_email, password, db);

    match item.first() {
        Some(item) => HttpResponse::Created().json(ResponseItem::new(
            ResponseStatus::Success,
            "Created new user".to_string(),
            UserItem::new(item.clone()),
        )),
        None => {
            // Logging a bit
            sentry::capture_message("Storing and lookup of new item failed!", Level::Error);

            HttpResponse::Conflict().json(ResponseItem::new(
                ResponseStatus::Error,
                "Error during user lookup and creation".to_string(),
                new_user_item,
            ))
        }
    }
}

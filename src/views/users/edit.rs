use crate::database::DB;
use crate::helpers::email::parse_email_from_string;
use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::response::item::Item as ResponseItem;
use crate::json_serialization::response::status::Status;
use crate::json_serialization::user::edit_item::EditItem;
use crate::json_serialization::user::item::Item as UserItem;
use crate::json_serialization::user::password_item::PasswordItem;
use crate::jwt::JwToken;
use crate::models::user::item::{edit as edit_item, update_password};
use actix_web::{web, HttpRequest, HttpResponse};
use sentry::Level;
use uuid::Uuid;

#[allow(clippy::future_not_send)]
pub async fn edit(user_item: web::Json<EditItem>, request: HttpRequest, db: DB, _: JwToken) -> HttpResponse {
    let uuid: Uuid = match parse_uuid_from_request(&request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    let username = String::from(&user_item.username);
    let email = String::from(&user_item.email);
    let salutation = String::from(&user_item.salutation);
    let first_name = String::from(&user_item.first_name);
    let last_name = String::from(&user_item.last_name);

    let valid_email: String = match parse_email_from_string(email) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    // Editing in DB
    let item = edit_item(uuid, username, valid_email, salutation, first_name, last_name, db);

    item.first().map_or_else(
        || {
            // Logging a bit
            sentry::capture_message("Editing and lookup of changed user failed!", Level::Error);

            HttpResponse::NotFound().json(ResponseItem::new(
                Status::Error,
                "User not found for".to_string(),
                user_item,
            ))
        },
        |item| {
            HttpResponse::Ok().json(ResponseItem::new(
                Status::Success,
                "Updated user".to_string(),
                UserItem::new(item),
            ))
        },
    )
}

#[allow(clippy::future_not_send)]
pub async fn password(
    user_item: web::Json<PasswordItem>,
    request: HttpRequest,
    db: DB,
    db2: DB,
    _: JwToken,
) -> HttpResponse {
    let uuid: Uuid = match parse_uuid_from_request(&request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    let old_password = &user_item.old_password;
    let new_password = &user_item.new_password;
    if new_password.is_empty() {
        return HttpResponse::UnprocessableEntity().json(ResponseItem::new(
            Status::Error,
            "Password constraint".to_string(),
            "New password must not be empty",
        ));
    }

    // Editing in DB
    let item = update_password(uuid, old_password, new_password, db, db2);

    item.map_or_else(
        || {
            HttpResponse::Conflict().json(ResponseItem::new(
                Status::Error,
                "User not found or password wrong".to_string(),
                user_item,
            ))
        },
        |user| {
            HttpResponse::Ok().json(ResponseItem::new(
                Status::Success,
                "Updated user password".to_string(),
                UserItem::new(&user),
            ))
        },
    )
}

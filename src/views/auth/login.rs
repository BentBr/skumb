use crate::database::DB;
use crate::json_serialization::response::item::Item;
use crate::json_serialization::response::status::Status;
use crate::json_serialization::user::auth::login::Login;
use crate::jwt::JwToken;
use crate::models::user::item::fetch_user_by_login;
use actix_web::{web, HttpResponse};
use std::collections::HashMap;

pub async fn login(credentials: web::Json<Login>, db: DB) -> HttpResponse {
    let password = &credentials.password;
    let email = &credentials.email;

    match fetch_user_by_login(email, password, db) {
        None => HttpResponse::Forbidden().json(Item::new(
            Status::Error,
            "Login failed".to_string(),
            "Check credentials: Email and password",
        )),
        Some(user) => {
            let new_token = JwToken::new(user.uuid);
            let raw_token = new_token.encode();
            let mut body = HashMap::new();
            body.insert("token", raw_token);

            HttpResponse::Created().json(Item::new(Status::Success, "Session token created".to_string(), body))
        }
    }
}

use crate::json_serialization::user::new_user_item::NewUserItem;
use crate::views::handlers::not_found_handler::not_found;
use crate::views::users::create;
use actix_web::web::{route, scope, ServiceConfig};
use actix_web::{web, HttpResponse};
use auth::auth_views_factory;
use lambda_http::{Body, Error, IntoResponse, Request, Response};
use serde_json::json;
use users::user_views_factory;

mod auth;
pub mod handlers;
mod users;

pub fn views_factory(app: &mut ServiceConfig) {
    //auth_views_factory(app);
    //user_views_factory(app);
    default_factory(app)
}

pub fn default_factory(app: &mut ServiceConfig) {
    app.service(scope("").default_service(route().to(not_found)));
}

async fn handle_create_route() -> HttpResponse {
    // Handle create route here
    // Call the appropriate function from your views
    // For example: create::create(event)
    create::create.await
}
/*
async fn handle_get_route(event: Request) -> Result<impl IntoResponse, lambda_http::Error> {
    // Handle get route here
    // Call the appropriate function from your views
    // For example: get::get(event)
}
*/
pub async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    /*

        // Extract some useful information from the request
        let who = event
            .query_string_parameters_ref()
            .and_then(|params| params.first("name"))
            .unwrap_or("world");
        let message = format!("Hello {who}, blaa HTTP request");

        // Return something that implements IntoResponse.
        // It will be serialized to the right response event automatically by the runtime
        let resp = Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(json!(message).to_string().into())
            .map_err(Box::new)?;
        Ok(resp)

    */

    let json_body = event.body().await.map_err(Box::new)?;

    match event.uri().path() {
        "/v1/user/create" => Ok(handle_create_route().await),
        //"/v1/user/get" => handle_get_route(event).await,
        // Add similar match arms for other routes
        path => {
            let resp = Response::builder()
                .status(404)
                .header("content-type", "application/json")
                .body(
                    format!(
                        json!({"status":"error","message":"Not found","data":"{}"}),
                        path
                    )
                    .to_string()
                    .into(),
                )
                .map_err(Box::new)?;
            Ok(resp)
        }
    }
}

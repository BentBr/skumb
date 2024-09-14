mod database;
mod helpers;
mod json_serialization;
mod jwt;
mod models;
mod schema;
mod views;
mod ws_actor;

use crate::helpers::env::get_float;
use crate::json_serialization::response::item::Item as ResponseItem;
use crate::json_serialization::response::status::Status;
use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::env;
use uuid::Uuid;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    create_sentry();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let chat_server = ws_actor::ChatServer::new().start();

    let server = HttpServer::new(move || {
        // Handling CORS issues
        let cors = Cors::default().allow_any_origin().allow_any_header().allow_any_method();

        App::new()
            // todo: handle auth here
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
            .wrap(cors)
            .app_data(web::Data::new(chat_server.clone()))
            // Websocket in general
            .service(web::resource("/ws/{chat_uuid}").route(web::get().to(ws_index)))
            // Api routes
            .service(web::resource("/health").route(web::get().to(health)))
            .configure(views::factory)
    })
    .bind(get_local_port_address())?
    .workers(1)
    .run();

    server.await?;
    Ok(())
}

fn create_sentry() {
    let sentry_dsn: String = env::var("SENTRY_DSN").expect("SENTRY_DSN not set");
    let sample_rate = get_float("SENTRY_SAMPLE_RATE");

    let _guard = sentry::init((
        sentry_dsn.as_str(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            // Placeholder for now - not yet enabled
            traces_sample_rate: sample_rate,
            ..Default::default()
        },
    ));
}

#[allow(clippy::future_not_send)]
async fn ws_index(
    request: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ws_actor::ChatServer>>,
    path: Option<web::Path<Uuid>>,
) -> Result<HttpResponse, actix_web::Error> {
    let chat_uuid = path.map_or_else(Uuid::new_v4, web::Path::into_inner);

    // Todo: getting user_uuid from jwt auth
    let user_uuid = Uuid::new_v4();

    ws::start(
        ws_actor::MyWs {
            chat_uuid,
            user_uuid,
            users: srv.get_ref().clone(),
        },
        &request,
        stream,
    )
}

#[allow(clippy::future_not_send)]
async fn health() -> Result<HttpResponse, actix_web::Error> {
    let test_uuid = Uuid::new_v4();

    let response = HttpResponse::Ok().json(ResponseItem::new(
        Status::Success,
        "Is healthy".to_string(),
        format!("Health check at {} with id: {}", chrono::Utc::now(), test_uuid),
    ));

    Ok(response)
}

fn get_local_port_address() -> String {
    let port = env::var("LOCAL_BE_PORT").expect("LOCAL_BE_PORT not set");

    format!("0.0.0.0:{port}")
}

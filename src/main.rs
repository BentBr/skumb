mod database;
mod helpers;
mod json_serialization;
mod jwt;
mod models;
mod schema;
mod views;
mod ws_actor;

use crate::helpers::env::get_float_from_env;
use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use futures::future::try_join3;
use std::env;
use actix_web::web::Path;
use uuid::Uuid;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    create_sentry();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server1 = HttpServer::new(|| {
        // Handling CORS issues
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        // Returning the app
        App::new()
            .configure(views::factory)
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
    })
    .bind("0.0.0.0:9123")?
    .workers(1)
    .run();

    let chat_server1 = ws_actor::ChatServer::new().start();

    let server2 = HttpServer::new(move || {
        App::new()
            // todo: handle auth here
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
            .app_data(web::Data::new(chat_server1.clone()))
            .service(web::resource("/ws/{chat_uuid}").route(web::get().to(ws_index)))
    })
    .bind("0.0.0.0:9124")?
    .workers(1)
    .run();

    let chat_server2 = ws_actor::ChatServer::new().start();

    let server3 = HttpServer::new(move || {
        App::new()
            // todo: handle auth here
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
            .app_data(web::Data::new(chat_server2.clone()))
            .service(web::resource("/ws").route(web::get().to(ws_index)))
    })
    .bind("0.0.0.0:9125")?
    .workers(1)
    .run();

    try_join3(server1, server2, server3).await?;
    Ok(())
}

fn create_sentry() {
    let sentry_dsn: String = env::var("SENTRY_DSN").expect("SENTRY_DSN not set");
    let sample_rate = get_float_from_env("SENTRY_SAMPLE_RATE".to_string());

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

async fn ws_index(
    request: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ws_actor::ChatServer>>,
    path: Option<web::Path<Uuid>>,
) -> Result<HttpResponse, actix_web::Error> {
    let chat_uuid = path.map_or_else(Uuid::new_v4, Path::into_inner);

    // Todo:: getting user_uuid from jwt auth
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

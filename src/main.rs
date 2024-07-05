mod database;
mod helpers;
mod json_serialization;
mod jwt;
mod models;
mod schema;
mod views;
mod ws_actor;

use crate::helpers::env::get_float_from_env;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use actix_web_actors::ws;
use std::env;
use futures::future::try_join;
use ws_actor::MyWs;

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
            .configure(views::views_factory)
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
    })
    .bind("0.0.0.0:9123")?
    .workers(1)
    .run();

    let server2 = HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D")) // Add logger middleware here
            .service(
                web::resource("/ws/")
                    .route(web::get().to(ws_index)),
            )
    })
        .bind("0.0.0.0:9124")?
        .workers(1)
        .run();

    try_join(server1, server2).await?;
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

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(MyWs {}, &r, stream)
}

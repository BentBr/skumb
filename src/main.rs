mod database;
mod helpers;
mod json_serialization;
mod jwt;
mod models;
mod schema;
mod views;

use crate::helpers::env::get_float_from_env;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use std::env;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    create_sentry();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
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
    .run()
    .await
}

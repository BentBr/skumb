mod database;
mod helpers;
mod json_serialization;
mod jwt;
mod models;
mod schema;
mod views;

use crate::helpers::env::get_float_from_env;
use crate::views::function_handler;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use futures::future;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use serde_json::json;
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
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    create_sentry();

    run(service_fn(function_handler)).await
}

mod auth;
mod chat;
pub mod handlers;
mod users;

use actix_web::web::{route, scope, ServiceConfig};
use auth::views_factory as auth_views_factory;
use chat::views_factory as chat_views_factory;
use handlers::not_found_handler::not_found;
use users::views_factory as user_views_factory;

pub fn factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    user_views_factory(app);
    chat_views_factory(app);
    default_factory(app);
}

pub fn default_factory(app: &mut ServiceConfig) {
    app.service(scope("").default_service(route().to(not_found)));
}

mod auth;
pub mod handlers;
mod users;

use crate::views::handlers::not_found_handler::not_found;
use actix_web::web::{route, scope, ServiceConfig};
use auth::auth_views_factory;
use users::user_views_factory;

pub fn factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    user_views_factory(app);
    default_factory(app);
}

pub fn default_factory(app: &mut ServiceConfig) {
    app.service(scope("").default_service(route().to(not_found)));
}

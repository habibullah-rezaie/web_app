use actix_web::web::ServiceConfig;

mod auth;
mod path;

pub fn views_factory(app: &mut ServiceConfig) {
    auth::auth_factory(app);
}

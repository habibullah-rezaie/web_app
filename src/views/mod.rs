use actix_web::web::ServiceConfig;

mod auth;
mod item;
mod path;

pub fn views_factory(app: &mut ServiceConfig) {
    auth::auth_factory(app);
    item::item_factory(app);
}

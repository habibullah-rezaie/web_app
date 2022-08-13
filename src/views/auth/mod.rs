use actix_web::web::{self, ServiceConfig};

use self::{login::login, logout::logout};

use super::path::Path;

pub mod login;
pub mod logout;

pub fn auth_factory(app: &mut ServiceConfig) {
    let auth_path = Path {
        prefix: "/auth/".to_string(),
    };

    app.route(&auth_path.define("login".to_string()), web::get().to(login));
    app.route(
        &auth_path.define("logout".to_string()),
        web::get().to(logout),
    );
}

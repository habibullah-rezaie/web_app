use actix_web::web::{self, ServiceConfig};

use crate::views::path::Path;

use self::{create::create, delete::delete, edit::edit, get::get};

mod create;
mod delete;
mod edit;
mod get;
mod utils;

pub fn item_factory(app: &mut ServiceConfig) {
    let item_path = Path {
        prefix: "/item".to_string(),
    };

    app.route(
        &item_path.define("/create/{title}".to_string()),
        web::post().to(create),
    );

    app.route(
        &item_path.define("/delete/{title}".to_string()),
        web::delete().to(delete),
    );

    app.route(&item_path.define("/get".to_string()), web::get().to(get));
    app.route(&item_path.define("/edit".to_string()), web::put().to(edit));
}

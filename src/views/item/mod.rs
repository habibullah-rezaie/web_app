use actix_web::web::{self, ServiceConfig};

use crate::views::path::Path;

use self::create::create;

mod create;

pub fn item_factory(app: &mut ServiceConfig) {
    let item_path = Path {
        prefix: "/item".to_string(),
    };

    app.route(
        &item_path.define("/create/{title}".to_string()),
        web::get().to(create),
    );
}

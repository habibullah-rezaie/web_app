use actix_web::HttpRequest;

use crate::{process::process_input, state::read_file, to_do::to_do_factory};

pub async fn create(req: HttpRequest) -> String {
    let state = read_file("state.json");

    let title: String = match req.match_info().get("title") {
        Some(value) => value.to_string(),
        None => panic!("Title not found!"),
    };

    let status = String::from("pending");

    let item = match to_do_factory(&status, &title) {
        Some(item) => item,
        None => panic!("{}", "Invalid status provided."),
    };

    // process input here
    process_input(item, "create".to_string(), &state);

    format!("Created todo {}", title)
}

use actix_web::{HttpRequest, HttpResponse, Responder};
use async_std::future::timeout;

use crate::{
    process::process_input,
    state::read_file,
    to_do::{to_do_factory, TodoTypes},
};

use super::utils::return_state;

pub async fn delete(req: HttpRequest) -> impl Responder {
    let title = req.match_info().get("title").unwrap();

    let state = read_file("state.json");

    let item: TodoTypes = match state.get(title) {
        None => return HttpResponse::NotFound().json(format!("{} not found in state", title)),
        Some(status_json) => {
            let status = status_json.to_string().replace('\"', "");
            to_do_factory(&status, title).unwrap()
        }
    };

    process_input(item, "delete".to_string(), &state);
    HttpResponse::Ok().json(return_state())
}

use actix_web::{web, HttpResponse};

use crate::{
    process::process_input,
    state::read_file,
    to_do::{structs::base::Todo, to_do_factory, TodoTypes},
};

use super::utils::return_state;

pub async fn edit(todo_item: web::Json<Todo>) -> HttpResponse {
    let state = read_file("state.json");

    let status_in_state: String;
    match state.get(&todo_item.title) {
        Some(status_json) => status_in_state = status_json.to_string().replace('\"', ""),
        None => {
            return HttpResponse::NotFound()
                .json(format!("{} not found in state!", &todo_item.title))
        }
    }

    // Check if status is the same as in state
    if status_in_state == todo_item.status {
        return HttpResponse::Ok().json(return_state());
    }

    // Check for status validity
    if &todo_item.status != "pending" && &todo_item.status != "done" {
        return HttpResponse::BadRequest()
            .json(format!("{} is not acceptable!", &todo_item.status));
    }

    let item: TodoTypes = match to_do_factory(&status_in_state, &todo_item.title) {
        Some(val) => val,
        None => {
            return HttpResponse::InternalServerError()
                .json("Something went wrong on the server!".to_string());
        }
    };

    process_input(item, "edit".to_string(), &state);
    HttpResponse::Ok().json(return_state())
}

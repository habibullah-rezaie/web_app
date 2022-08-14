use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};

use crate::to_do::{structs::base::Todo, TodoTypes};

#[derive(serde::Serialize)]
pub struct TodoItems {
    done_items: Vec<Todo>,
    pending_items: Vec<Todo>,
    pending_items_count: u16,
    done_items_count: u16,
}

impl TodoItems {
    pub fn new(items: Vec<TodoTypes>) -> Self {
        let mut done_items = Vec::new();
        let mut pending_items = Vec::new();

        for item in items.iter() {
            match item {
                TodoTypes::Done(item) => done_items.push(Todo::new(
                    &item.super_struct.title,
                    &item.super_struct.status,
                )),
                TodoTypes::Pending(item) => pending_items.push(Todo::new(
                    &item.super_struct.title,
                    &item.super_struct.status,
                )),
            }
        }

        let pending_items_count = pending_items.len() as u16;
        let done_items_count = done_items.len() as u16;
        TodoItems {
            done_items,
            pending_items,
            pending_items_count,
            done_items_count,
        }
    }
}

impl Responder for TodoItems {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

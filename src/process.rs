use serde_json::{Map, Value};

use crate::to_do::{
    structs::{
        done::DoneTodo,
        pending::PendingTodo,
        traits::{create::Create, delete::Delete, edit::Edit, get::Get},
    },
    TodoTypes,
};

pub fn process_input(item: TodoTypes, command: String, state: &Map<String, Value>) {
    if command.is_empty() {
        return;
    }

    let mut state = state.clone();
    match item {
        TodoTypes::Done(item) => process_done(item, command, &mut state),
        TodoTypes::Pending(item) => process_pending(item, command, &mut state),
    }
}

pub fn process_done(item: DoneTodo, command: String, state: &mut Map<String, Value>) {
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, state),
        "delete" => item.delete(&item.super_struct.title, state),
        "edit" => item.set_to_pending(&item.super_struct.title, state),
        _ => println!("command: {} not supported", command),
    }
}

pub fn process_pending(item: PendingTodo, command: String, state: &mut Map<String, Value>) {
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, state),
        "delete" => item.delete(&item.super_struct.title, state),
        "edit" => item.set_to_done(&item.super_struct.title, state),
        "create" => item.create(&item.super_struct.title, &item.super_struct.status, state),
        _ => println!("command: {} not supported", command),
    }
}

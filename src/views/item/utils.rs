use crate::{
    json_serializaitions::todo_items::TodoItems,
    state::read_file,
    to_do::{to_do_factory, TodoTypes},
};

pub fn return_state() -> TodoItems {
    let state = read_file("state.json");

    let mut todo_items: Vec<TodoTypes> = Vec::new();

    for key in state.keys() {
        match state.get(key) {
            Some(value) => match value.to_string().replace('\"', "").as_str() {
                "pending" => todo_items.push(to_do_factory("pending", key).unwrap()),
                "done" => todo_items.push(to_do_factory("done", key).unwrap()),
                _ => panic!("Invalid todo type detected"),
            },
            None => panic!("Something went wrong."),
        };
    }

    TodoItems::new(todo_items)
}

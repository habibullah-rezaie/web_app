use self::structs::{done::DoneTodo, pending::PendingTodo};

pub mod structs;

pub enum TodoTypes {
    Done(DoneTodo),
    Pending(PendingTodo),
}

pub fn to_do_factory(todo_type: &str, title: &str) -> Option<TodoTypes> {
    if todo_type == "done" {
        let todo = DoneTodo::new(title);
        return Some(TodoTypes::Done(todo));
    } else if todo_type == "pending" {
        let todo = PendingTodo::new(title);
        return Some(TodoTypes::Pending(todo));
    }

    None
}

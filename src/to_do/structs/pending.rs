use super::{
    base::Todo,
    traits::{create::Create, delete::Delete, edit::Edit, get::Get},
};

pub struct PendingTodo {
    pub super_struct: Todo,
}

impl PendingTodo {
    pub fn new(title: &str) -> Self {
        PendingTodo {
            super_struct: Todo::new(title, "pending"),
        }
    }
}

impl Edit for PendingTodo {}
impl Delete for PendingTodo {}
impl Get for PendingTodo {}
impl Create for PendingTodo {}

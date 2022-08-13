use super::{
    base::Todo,
    traits::{delete::Delete, edit::Edit, get::Get},
};

pub struct DoneTodo {
    pub super_struct: Todo,
}

impl DoneTodo {
    pub fn new(title: &str) -> Self {
        DoneTodo {
            super_struct: Todo::new(title, "done"),
        }
    }
}

impl Edit for DoneTodo {}
impl Delete for DoneTodo {}
impl Get for DoneTodo {}

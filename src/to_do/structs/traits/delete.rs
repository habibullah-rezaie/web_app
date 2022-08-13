use serde_json::{Map, Value};

use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        state.remove(title).expect("Todo not found.");

        write_to_file("state.json", state);

        println!("{} sucessfully deleted", title);
    }
}

use serde_json::{json, Map, Value};

use crate::state::write_to_file;

pub trait Create {
    fn create(&self, title: &str, status: &str, state: &mut Map<String, Value>) {
        match state.insert(title.to_string(), json!(status.to_string())) {
            Some(_) => {
                panic!("\nThis todo already exists. Aborting.");
            }
            None => {}
        }

        write_to_file("state.json", state);

        println!("\n\n{} is being created.", title);
    }
}

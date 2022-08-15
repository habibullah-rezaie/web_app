use serde_json::{json, Map, Value};

use crate::state::write_to_file;

pub trait Edit {
    fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
        let status = state.get(title).expect("Todo not found.");

        match status.as_str().unwrap() {
            "pending" => {
                state.insert(title.to_string(), json!("done"));
                write_to_file("state.json", state);

                println!("{} is set to done", title);
            }
            "done" => {
                println!("{} is already done", title);
            }
            _ => panic!("{}", "Something went badly wrong!"),
        }
    }

    fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        let status = state.get(title).expect("Todo not found.");

        match status.as_str().unwrap() {
            "done" => {
                state.insert(title.to_string(), json!("pending"));
                write_to_file("state.json", state);

                println!("{} is set to pending", title);
            }
            "pending" => {
                println!("{} is already pending", title);
            }
            _ => panic!("{}", "Something went badly wrong!"),
        }
    }
}

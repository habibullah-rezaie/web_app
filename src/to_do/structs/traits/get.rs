use serde_json::{Map, Value};

pub trait Get {
    fn get(&self, title: &str, state: &Map<String, Value>) {
        let todo: &Value = match state.get(&title.to_string()) {
            Some(json_val) => json_val,
            None => panic!("Todo not found {}", title),
        };

        let todo_string = todo.as_str().unwrap();

        println!("title: {}, status: {}", title, todo_string);
    }
}

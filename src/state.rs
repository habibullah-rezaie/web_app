use std::{
    fs::{write, File},
    io::Read,
};

use serde_json::{Map, Value};

pub fn write_to_file(filename: &str, state: &mut Map<String, Value>) {
    let json = serde_json::json!(state);

    write(filename, json.to_string()).expect("Something went wrong saving state.");
}

pub fn read_file(filename: &str) -> Map<String, Value> {
    let mut file = File::open(filename).unwrap();

    let mut json_string = String::new();

    file.read_to_string(&mut json_string).unwrap();

    let json_data: Value = serde_json::from_str(&json_string).unwrap();

    return json_data.as_object().unwrap().clone();
}

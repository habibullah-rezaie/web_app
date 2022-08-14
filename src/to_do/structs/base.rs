#[derive(serde::Serialize)]
pub struct Todo {
    pub title: String,
    pub status: String,
}

impl Todo {
    pub fn new(title: &str, status: &str) -> Self {
        Todo {
            title: title.to_string(),
            status: status.to_string(),
        }
    }
}

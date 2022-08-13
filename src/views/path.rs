pub struct Path {
    pub prefix: String,
}

impl Path {
    pub fn define(&self, path: String) -> String {
        self.prefix.to_owned() + &path
    }
}

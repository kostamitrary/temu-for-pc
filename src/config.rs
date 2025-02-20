pub struct Config {
    pub application_path: String,
}

impl Config {
    pub fn new(path: &str) -> Self {
        Config {
            application_path: path.to_string(),
        }
    }
}
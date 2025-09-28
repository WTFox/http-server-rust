#[derive(Clone)]
pub struct AppConfig {
    pub directory: Option<String>,
    pub supported_encodings: Vec<String>,
}

impl AppConfig {
    pub fn new(directory: Option<String>) -> Self {
        AppConfig {
            directory: directory,
            supported_encodings: vec!["text/plain".to_string(), "gzip".to_string()],
        }
    }
}

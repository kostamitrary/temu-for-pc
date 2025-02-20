pub fn validate_path(path: &str) -> bool {
    std::path::Path::new(path).exists()
}
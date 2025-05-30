



pub struct UrlConfig {
    pub base_url: String,
    
}

impl Default for UrlConfig {
    fn default() -> Self {
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        UrlConfig {
            base_url: base_url,
        }
    }
}
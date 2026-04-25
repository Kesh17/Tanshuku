#[derive(Debug)]
pub struct Config {
    pub base_url: String,
    pub port: String,
}

impl Config {
    pub fn build() -> Self {
        Self {
            base_url: std::env::var("BASE_URL").unwrap_or_else(|_| format!("localhost")),
            port: std::env::var("PORT").unwrap_or_else(|_| "3000".to_string()),
        }
    }

    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.base_url, self.port)
    }

    pub fn get_url(&self) -> String {
        format!("http://{}:{}/", self.base_url, self.port)
    }
}

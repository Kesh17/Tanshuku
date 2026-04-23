use url::Url;

pub fn generate_short_code<T>(uid: T) -> String
where
    T: Into<u128>,
{
    base62::encode(uid)
}

pub fn generate_short_url(short_code: &str) -> Result<Url, url::ParseError> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let base_url =
        std::env::var("BASE_URL").unwrap_or_else(|_| format!("http://localhost:{}/", port));
    Url::parse(&format!("{}{}", base_url, short_code))
}

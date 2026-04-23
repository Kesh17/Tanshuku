use axum::response::Html;
use serde::{Deserialize, Serialize};
use url::Url;

pub async fn get_index() -> Html<String> {
    match tokio::fs::read_to_string("static/index.html").await {
        Ok(content) => Html(content),
        Err(_) => Html(String::from("Page not found")),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUrl {
    url: Url,
}

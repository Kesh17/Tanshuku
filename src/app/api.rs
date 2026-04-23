use axum::response::Html;

pub async fn get_index() -> Html<String> {
    match tokio::fs::read_to_string("static/index.html").await {
        Ok(content) => Html(content),
        Err(_) => Html(String::from("Page not found")),
    }
}

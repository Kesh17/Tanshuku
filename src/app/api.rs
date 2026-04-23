use axum::{
    Json,
    extract::{Query, State},
    http,
    response::{Html, IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use url::Url;

pub async fn get_index() -> Html<String> {
    match tokio::fs::read_to_string("static/index.html").await {
        Ok(content) => Html(content),
        Err(_) => Html(String::from("Page not found")),
    }
}

#[derive(Deserialize, Debug)]
pub struct UrlRequest {
    url: Url,
}

#[derive(Serialize)]
pub enum UrlResponse {
    Message(String),
    Url(String),
}

impl IntoResponse for UrlResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            UrlResponse::Message(_) => http::StatusCode::NOT_FOUND,
            UrlResponse::Url(_) => http::StatusCode::OK,
        };
        (status_code, Json(self)).into_response()
    }
}

#[axum::debug_handler]
pub async fn get_short_url(
    State(state): State<sqlx::PgPool>,
    Query(payload): Query<UrlRequest>,
) -> impl IntoResponse {
    let url = sqlx::query!(
        "SELECT long_url FROM short_url WHERE long_url = $1;",
        payload.url.as_str()
    )
    .fetch_one(&state)
    .await;
    match url {
        Ok(val) => UrlResponse::Url(val.long_url),
        Err(_) => UrlResponse::Message("URL not found".to_string()),
    }
}

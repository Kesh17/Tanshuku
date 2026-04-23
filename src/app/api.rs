use axum::{
    Json,
    extract::{Query, State},
    http,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::app::model::ShortUrl;

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

#[derive(Serialize, Debug)]
pub enum UrlResponse {
    Message(String),
    ShortUrl(ShortUrl),
}

impl IntoResponse for UrlResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            UrlResponse::Message(_) => http::StatusCode::NOT_FOUND,
            UrlResponse::ShortUrl(_) => http::StatusCode::OK,
        };
        (status_code, Json(self)).into_response()
    }
}

#[axum::debug_handler]
pub async fn get_short_url(
    State(state): State<sqlx::PgPool>,
    Query(payload): Query<UrlRequest>,
) -> impl IntoResponse {
    let short_url = sqlx::query!(
        "SELECT id, long_url, short_code, short_url FROM short_urls WHERE short_url = $1;",
        payload.url.as_str()
    )
    .map(|x| {
        ShortUrl::new(
            x.id as i64,
            Url::parse(&x.long_url).unwrap(),
            x.short_code,
            Url::parse(&x.short_url).unwrap(),
        )
    })
    .fetch_one(&state)
    .await;
    match short_url {
        Ok(val) => UrlResponse::ShortUrl(val),
        Err(_) => UrlResponse::Message("URL not found".to_string()),
    }
}

pub async fn set_short_url(
    State(state): State<sqlx::PgPool>,
    Json(payload): Json<UrlRequest>,
) -> impl IntoResponse {
    let Ok(id) = sqlx::query!("SELECT nextval('id_seq') as id;")
        .fetch_one(&state)
        .await
    else {
        return UrlResponse::Message("Failed to generate ID".to_string());
    };
    let Ok(short_url_map) = ShortUrl::build(id.id.unwrap(), payload.url.clone()) else {
        return UrlResponse::Message("Failed to create short URL".to_string());
    };

    let row = sqlx::query!(
        r#"
            INSERT INTO short_urls (id, long_url, short_url, short_code)
            VALUES ($1, $2, $3, $4);
        "#,
        short_url_map.id,
        short_url_map.long_url.as_str(),
        short_url_map.short_url.as_str(),
        short_url_map.short_code
    )
    .execute(&state)
    .await;

    match row {
        Ok(_) => UrlResponse::ShortUrl(short_url_map),
        Err(_) => UrlResponse::Message("Failed to create short URL".to_string()),
    }
}

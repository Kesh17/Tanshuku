use axum::{
    Json,
    extract::{Query, State},
    http,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::app::{AppState, model::ShortUrl, utils};

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
    State(state): State<AppState>,
    Query(payload): Query<UrlRequest>,
) -> UrlResponse {
    let short_code = payload
        .url
        .as_str()
        .replace(state.config.get_url().as_str(), "");
    // println!("{}", short_code);

    match ShortUrl::get_url_from_db(&state.db, &short_code).await {
        Ok(Some(val)) => UrlResponse::ShortUrl(val),
        Ok(None) => UrlResponse::Message("Url not found".to_string()),
        Err(_) => UrlResponse::Message("Database Error".to_string()),
    }
}

pub async fn set_short_url(
    State(state): State<AppState>,
    Json(payload): Json<UrlRequest>,
) -> UrlResponse {
    let short_code = &utils::generate_hash(payload.url.as_str())[..7];
    match ShortUrl::get_url_from_db(&state.db, short_code).await {
        Ok(Some(val)) => {
            // println!("Value present");
            UrlResponse::ShortUrl(val)
        }
        Ok(None) => {
            let Ok(short_url_map) = ShortUrl::build(&state.db, &state.config, payload.url).await
            else {
                return UrlResponse::Message("Database Error".to_string());
            };
            let Ok(_) = sqlx::query!(
                r#"
                    INSERT INTO short_urls (long_url, short_url, short_code)
                    VALUES ($1, $2, $3);
                "#,
                short_url_map.long_url.as_str(),
                short_url_map.short_url.as_str(),
                short_url_map.short_code
            )
            .execute(&state.db)
            .await
            else {
                return UrlResponse::Message("Database Error".to_string());
            };

            UrlResponse::ShortUrl(short_url_map)
        }
        Err(_) => UrlResponse::Message("Database Error".to_string()),
    }
}

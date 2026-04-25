use axum::{
    Json,
    extract::State,
    response::{Html, Redirect},
};
use serde::Deserialize;
use url::Url;

use crate::app::{
    AppState,
    error::{AppError, AppQuery},
    model::ShortUrl,
    utils,
};

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

#[axum::debug_handler]
pub async fn get_short_url(
    State(state): State<AppState>,
    AppQuery(payload): AppQuery<UrlRequest>,
) -> Result<Redirect, AppError> {
    let short_code = payload
        .url
        .as_str()
        .replace(state.config.get_url().as_str(), "");

    match ShortUrl::get_url_from_db(&state.db, &short_code).await {
        Ok(Some(val)) => Ok(Redirect::temporary(val.long_url.as_str())),
        Ok(None) => Err(AppError::URLNotFound),
        Err(e) => Err(e),
    }
}

#[axum::debug_handler]
pub async fn set_short_url(
    State(state): State<AppState>,
    Json(payload): Json<UrlRequest>,
) -> Result<Json<ShortUrl>, AppError> {
    let short_code = &utils::generate_hash(payload.url.as_str())[..7];
    match ShortUrl::get_url_from_db(&state.db, short_code).await {
        Ok(Some(val)) => Ok(Json(val)),
        Ok(None) => {
            let short_url_map = ShortUrl::build(&state.db, &state.config, payload.url).await?;
            let _ = sqlx::query!(
                r#"
                    INSERT INTO short_urls (long_url, short_url, short_code)
                    VALUES ($1, $2, $3);
                "#,
                short_url_map.long_url.as_str(),
                short_url_map.short_url.as_str(),
                short_url_map.short_code
            )
            .execute(&state.db)
            .await?;

            Ok(Json(short_url_map))
        }
        Err(e) => Err(e),
    }
}

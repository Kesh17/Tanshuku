use axum::extract::State;
use url::Url;

use crate::app::utils;

#[derive(Debug)]
pub struct ShortUrl {
    id: u64,
    long_url: Url,
    short_code: String,
}

impl ShortUrl {
    async fn build(
        State(state): State<sqlx::PgPool>,
        long_url: Url,
    ) -> Result<ShortUrl, Box<dyn std::error::Error>> {
        let id = sqlx::query!("SELECT nextval('id_seq') as id;")
            .fetch_one(&state)
            .await?
            .id
            .unwrap() as u64;
        Ok(Self {
            id,
            long_url,
            short_code: utils::generate_short_code(&id),
        })
    }
}

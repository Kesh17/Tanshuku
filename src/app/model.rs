use serde::Serialize;
use sqlx::prelude::FromRow;
use url::Url;

use crate::app::{config::Config, error::AppError, utils};

#[derive(Serialize, Debug, FromRow)]
pub struct ShortUrl {
    pub long_url: Url,
    pub short_code: String,
    pub short_url: Url,
}

impl ShortUrl {
    pub fn new(long_url: Url, short_code: String, short_url: Url) -> Self {
        Self {
            long_url,
            short_code,
            short_url,
        }
    }

    pub async fn build(
        pool: &sqlx::PgPool,
        config: &Config,
        long_url: Url,
    ) -> Result<Self, AppError> {
        let short_code = utils::generate_short_code(pool, long_url.as_str()).await?;
        let short_url = utils::generate_short_url(&short_code, &config.get_url())?;
        Ok(Self {
            long_url,
            short_code,
            short_url: short_url,
        })
    }

    pub async fn get_url_from_db(
        pool: &sqlx::PgPool,
        short_code: &str,
    ) -> Result<Option<Self>, AppError> {
        sqlx::query!(
            "SELECT long_url, short_code, short_url FROM short_urls WHERE short_code = $1",
            short_code
        )
        .map(|x| {
            ShortUrl::new(
                Url::parse(&x.long_url).unwrap(),
                x.short_code,
                Url::parse(&x.short_url).unwrap(),
            )
        })
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::DataBaseError)
    }
}

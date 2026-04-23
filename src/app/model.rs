use serde::Serialize;
use sqlx::prelude::FromRow;
use url::{ParseError, Url};

use crate::app::utils;

#[derive(Serialize, Debug, FromRow)]
pub struct ShortUrl {
    pub id: i64,
    pub long_url: Url,
    pub short_code: String,
    pub short_url: Url,
}

impl ShortUrl {
    pub fn new(id: i64, long_url: Url, short_code: String, short_url: Url) -> Self {
        Self {
            id,
            long_url,
            short_code,
            short_url,
        }
    }

    pub fn build(id: i64, long_url: Url) -> Result<Self, ParseError> {
        let short_code = utils::generate_short_code(id as u64);
        let short_url = utils::generate_short_url(&short_code)?;
        Ok(Self {
            id,
            long_url,
            short_code,
            short_url: short_url,
        })
    }
}

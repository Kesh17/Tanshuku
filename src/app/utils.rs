use hex;
use sha2::{Digest, Sha256};
use url::Url;

use crate::app::{error::AppError, model::ShortUrl};

pub fn generate_hash(long_url: &str) -> String {
    let hash = Sha256::digest(long_url);
    hex::encode(hash).to_string()
}
pub async fn generate_short_code(pool: &sqlx::PgPool, long_url: &str) -> Result<String, AppError> {
    let hash = generate_hash(long_url);

    let result = 'outer: {
        for window in hash.as_bytes().windows(7) {
            let short_code = String::from_utf8(window.to_vec()).map_err(|_| AppError::HashError)?;
            let db_url = ShortUrl::get_url_from_db(&pool, &short_code).await?;
            if db_url.is_none() {
                break 'outer Ok(short_code);
            }
        }
        Err(AppError::HashError)
    };
    result
}

pub fn generate_short_url(short_code: &str, base_url: &str) -> Result<Url, AppError> {
    Ok(Url::parse(&format!("{}{}", base_url, short_code))?)
}

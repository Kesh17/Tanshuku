#[derive(Debug)]
pub enum AppError {
    DataBaseError(sqlx::Error),
    HashError,
    ParseError(url::ParseError),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DataBaseError(err)
    }
}

impl From<url::ParseError> for AppError {
    fn from(err: url::ParseError) -> Self {
        AppError::ParseError(err)
    }
}

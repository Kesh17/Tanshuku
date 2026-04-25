use std::sync::Arc;

use axum::{
    Json,
    extract::{
        FromRequest, FromRequestParts, Query,
        rejection::{JsonRejection, QueryRejection},
    },
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    DataBaseError(sqlx::Error),
    JsonRejection(JsonRejection),
    QueryRejection(QueryRejection),
    URLNotFound,
    HashError,
    ParseError(url::ParseError),
}

#[derive(FromRequestParts)]
#[from_request(via(Query), rejection(AppError))]
pub struct AppQuery<T>(pub T);

impl From<JsonRejection> for AppError {
    fn from(err: JsonRejection) -> Self {
        AppError::JsonRejection(err)
    }
}

impl From<QueryRejection> for AppError {
    fn from(err: QueryRejection) -> Self {
        AppError::QueryRejection(err)
    }
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

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message, err) = match &self {
            AppError::JsonRejection(rejection) => (rejection.status(), rejection.body_text(), None),
            AppError::QueryRejection(rejection) => {
                (rejection.status(), rejection.body_text(), None)
            }
            AppError::DataBaseError(_e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_owned(),
                Some(self),
            ),

            AppError::HashError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Hashing error".to_owned(),
                Some(self),
            ),
            AppError::ParseError(_e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Parsing error".to_owned(),
                None,
            ),
            AppError::URLNotFound => (StatusCode::NOT_FOUND, "URL not found".to_owned(), None),
        };
        let mut response = (status, Json(ErrorResponse { message })).into_response();
        if let Some(err) = err {
            response.extensions_mut().insert(Arc::new(err));
        }
        response
    }
}

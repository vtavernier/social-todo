use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Error, Display, Serialize)]
pub enum ModelError {
    #[display("database error: {0}")]
    #[serde(serialize_with = "use_display")]
    Sqlx(#[error(source)] sqlx::Error),
    #[display("record not found")]
    NotFound,
}

fn use_display<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: std::fmt::Display,
    S: serde::ser::Serializer,
{
    serializer.collect_str(value)
}

impl From<sqlx::Error> for ModelError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Sqlx(other),
        }
    }
}

impl ResponseError for ModelError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ModelError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ModelError::NotFound => HttpResponse::NotFound().finish(),
            other => HttpResponse::InternalServerError().json(json!({ "error": other })),
        }
    }
}

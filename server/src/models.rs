// Some query methods are not used yet
#![allow(dead_code)]

use actix_web::{HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum ModelError {
    #[error("database error: {0}")]
    #[serde(serialize_with = "use_display")]
    Sqlx(#[source] sqlx::Error),
    #[error("record not found")]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(rename_all = "lowercase", type_name = "user_role")]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub role: UserRole,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub last_login_at: Option<chrono::NaiveDateTime>,
}

impl User {
    pub async fn find_all(pg_pool: &sqlx::PgPool) -> Result<Vec<Self>, ModelError> {
        Ok(sqlx::query_as::<_, Self>("SELECT * FROM users ORDER BY id")
            .fetch_all(pg_pool)
            .await?)
    }

    pub fn as_details(&self) -> UserDetails {
        UserDetails {
            id: self.id,
            name: self.name.clone(),
            role: self.role,
            created_at: self.created_at,
        }
    }
}

impl Responder for User {
    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        HttpResponse::Ok().json(&self)
    }
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
    pub id: i32,
    pub name: String,
    pub role: UserRole,
    pub created_at: chrono::NaiveDateTime,
}

impl UserDetails {
    pub async fn find(pg_pool: &sqlx::PgPool, id: i32) -> Result<Self, ModelError> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT id, name, role, created_at FROM users WHERE id = $1")
                .bind(id)
                .fetch_one(pg_pool)
                .await?,
        )
    }

    pub async fn find_all(pg_pool: &sqlx::PgPool) -> Result<Vec<Self>, ModelError> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT id, name, role, created_at FROM users ORDER BY id")
                .fetch_all(pg_pool)
                .await?,
        )
    }
}

impl Responder for UserDetails {
    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        HttpResponse::Ok().json(&self)
    }
}

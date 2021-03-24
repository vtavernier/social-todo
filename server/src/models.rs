// Some query methods are not used yet
#![allow(dead_code)]

mod connector;
pub use connector::*;

use actix_web::{HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, derive_more::Error, derive_more::Display, Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(rename_all = "lowercase", type_name = "user_role")]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    User,
    Admin,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    sqlx::Type,
    derive_more::Display,
)]
#[sqlx(transparent)]
#[display("{0}")]
pub struct UserId(pub i32);

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub password: String,
    pub role: UserRole,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub last_login_at: Option<chrono::NaiveDateTime>,
}

impl User {
    pub async fn find_all(conn: &Connector) -> Result<Vec<Self>, ModelError> {
        Ok(sqlx::query_as::<_, Self>("SELECT * FROM users ORDER BY id")
            .fetch_all(&conn.pg_pool)
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
    pub id: UserId,
    pub name: String,
    pub role: UserRole,
    pub created_at: chrono::NaiveDateTime,
}

impl UserDetails {
    pub async fn find(conn: &Connector, id: i32) -> Result<Self, ModelError> {
        Ok(conn
            .cached(&format!("user_details:{}", id), |pg_pool| async move {
                sqlx::query_as::<_, Self>(
                    "SELECT id, name, role, created_at FROM users WHERE id = $1",
                )
                .bind(id)
                .fetch_one(pg_pool)
                .await
            })
            .await?)
    }

    pub async fn find_all(conn: &Connector) -> Result<Vec<Self>, ModelError> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT id, name, role, created_at FROM users ORDER BY id")
                .fetch_all(&conn.pg_pool)
                .await?,
        )
    }
}

impl Responder for UserDetails {
    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        HttpResponse::Ok().json(&self)
    }
}

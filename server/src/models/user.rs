use actix_web::{http::StatusCode, web::HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};

use super::{Connector, ModelError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize, Deserialize)]
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
    Type,
    derive_more::Display,
)]
#[sqlx(transparent)]
#[display("{0}")]
pub struct UserId(pub i32);

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
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

#[derive(Debug, Error, Display, From)]
pub enum LoginError {
    #[display("invalid user credentials")]
    InvalidCredentials,
    #[display("internal error: {0}")]
    InternalError(#[error(source)] ModelError),
}

impl ResponseError for LoginError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            LoginError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            LoginError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(self.status_code())
    }
}

impl User {
    pub async fn find_all(conn: &Connector) -> Result<Vec<Self>, ModelError> {
        Ok(sqlx::query_as::<_, Self>("SELECT * FROM users ORDER BY id")
            .fetch_all(&conn.pg_pool)
            .await?)
    }

    pub async fn find_by_name(conn: &Connector, name: &str) -> Result<Self, ModelError> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM users WHERE name = $1")
                .bind(name)
                .fetch_one(&conn.pg_pool)
                .await?,
        )
    }

    pub async fn login(conn: &Connector, name: &str, password: &str) -> Result<Self, LoginError> {
        // Get the full user record for the given name
        let user = Self::find_by_name(conn, name)
            .await
            .map_err(|err| match err {
                ModelError::NotFound => LoginError::InvalidCredentials,
                other => LoginError::InternalError(other),
            })?;

        // Check the bcrypt password
        let bcrypt_result = bcrypt::verify(password, &user.password).map_err(|err| {
            warn!(%err, "bcrypt error");
            LoginError::InvalidCredentials
        })?;

        if bcrypt_result {
            Ok(user)
        } else {
            Err(LoginError::InvalidCredentials)
        }
    }

    pub fn into_details(self) -> UserDetails {
        UserDetails {
            id: self.id,
            name: self.name,
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

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_user_fetch_all() {
        actix_rt::System::new().block_on(async {
            let conn = get_connector().await;
            assert!(!User::find_all(&conn).await.unwrap().is_empty())
        })
    }
}

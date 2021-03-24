use actix_web::{web::HttpResponse, Responder};
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

use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

use crate::models;

#[get("/")]
async fn index(pg_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    models::UserDetails::find_all(pg_pool.get_ref())
        .await
        .map(|users| HttpResponse::Ok().json(json!({ "users": users })))
}

#[get("/{id}/")]
async fn show(pg_pool: web::Data<sqlx::PgPool>, id: web::Path<i32>) -> impl Responder {
    models::UserDetails::find(pg_pool.get_ref(), id.into_inner()).await
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(index).service(show));
}

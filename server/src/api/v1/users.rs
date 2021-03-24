use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

use crate::models;

#[get("/")]
async fn index(conn: web::Data<models::Connector>) -> impl Responder {
    models::UserDetails::find_all(conn.get_ref())
        .await
        .map(|users| HttpResponse::Ok().json(json!({ "users": users })))
}

#[get("/{id}/")]
async fn show(conn: web::Data<models::Connector>, id: web::Path<i32>) -> impl Responder {
    models::UserDetails::find(conn.get_ref(), id.into_inner()).await
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(index).service(show));
}

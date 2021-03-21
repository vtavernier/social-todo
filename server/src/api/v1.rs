use actix_web::{get, web, HttpResponse, Result};
use serde_json::json;

mod users;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({ "version": VERSION })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").service(index).configure(users::config));
}

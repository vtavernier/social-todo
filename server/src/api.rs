use actix_web::web;

mod v1;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(v1::config));
}

use actix_session::Session;
use actix_web::{delete, get, post, web, Error, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

use social_todo_server::models;

#[get("/")]
async fn index(
    conn: web::Data<models::Connector>,
    session: Session,
) -> Result<HttpResponse, Error> {
    // Get user id from session
    let user_id = session.get::<models::UserId>("user_id")?;

    match user_id {
        Some(user_id) => {
            match models::UserDetails::find(conn.get_ref(), user_id.0).await {
                Ok(user) => Ok(HttpResponse::Ok().json(json!({ "user": user }))),
                Err(models::ModelError::NotFound) => {
                    // User not found, deauth immediately
                    session.purge();
                    Ok(HttpResponse::Ok()
                        .json(json!({ "user": Option::<models::UserDetails>::None })))
                }
                Err(error) => Err(error)?,
            }
        }
        None => Ok(HttpResponse::Ok().json(json!({ "user": Option::<models::UserDetails>::None }))),
    }
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    name: String,
    password: String,
}

#[post("/")]
async fn login(
    conn: web::Data<models::Connector>,
    session: Session,
    login_request: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    // Try to authenticate the user
    let user =
        models::User::login(conn.get_ref(), &login_request.name, &login_request.password).await?;

    // On success, add the user id to the session
    session.insert("user_id", user.id)?;

    // Return the user object in the response
    Ok(HttpResponse::Ok().json(json!({ "user": user.into_details() })))
}

#[delete("/")]
async fn logout(session: Session) -> impl Responder {
    // Just clear the session
    session.purge();
    HttpResponse::Ok().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(index)
            .service(login)
            .service(logout),
    );
}

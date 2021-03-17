#[macro_use]
extern crate log;

use std::path::PathBuf;

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Result};
use serde_json::json;
use structopt::StructOpt;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({ "version": VERSION })))
}

#[derive(StructOpt)]
struct Opts {
    /// Increase logging verbosity
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u32,

    /// Bind address
    #[structopt(short, long, default_value = "127.0.0.1:8880")]
    bind: String,

    /// Path to static files to serve as root
    #[structopt(short, long)]
    webroot: Option<PathBuf>,
}

fn resolve_webroot(webroot: &Option<PathBuf>) -> std::io::Result<PathBuf> {
    if let Some(path) = webroot {
        Ok(path.clone())
    } else {
        Ok(std::env::current_exe()?
            .parent()
            .expect("failed to find exe directory")
            .join("../../client/dist"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load options
    let opts = Opts::from_args();

    // Initialize logger
    env_logger::Builder::from_env(
        env_logger::Env::new()
            .filter_or("SOCIAL_TODO_LOG", {
                let level = match opts.verbose {
                    0 => "warn",
                    1 => "info",
                    2 => "debug",
                    _ => "trace",
                };

                format!(
                    "actix_web::middleware::logger={level},social_todo_server={level}",
                    level = level
                )
            })
            .write_style("SOCIAL_TODO_LOG_STYLE"),
    )
    .format_timestamp(None)
    .try_init()
    .ok();

    let server = HttpServer::new({
        let webroot = match std::fs::canonicalize(resolve_webroot(&opts.webroot)?) {
            Ok(path) => {
                info!("resolved webroot to {}", path.display());
                Some(path)
            }
            Err(error) => {
                warn!("could not resolve webroot: {}", error);
                None
            }
        };

        move || {
            let app = App::new()
                .wrap(middleware::Logger::default())
                .service(web::scope("/api/v1").service(index));

            if let Some(webroot) = &webroot {
                app.service(actix_files::Files::new("/", &webroot).index_file("index.html"))
            } else {
                app
            }
        }
    })
    .bind(&opts.bind)?;

    info!("social-todo-server running at http://{}", &opts.bind);

    server.run().await
}

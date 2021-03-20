#[macro_use]
extern crate tracing;

use std::path::PathBuf;

use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use serde_json::json;
use structopt::StructOpt;

use tracing_subscriber::util::SubscriberInitExt;

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
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_env("SOCIAL_TODO_LOG").unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::from_default_env().add_directive(
                    match opts.verbose {
                        0 => "warn",
                        1 => "info",
                        2 => "debug",
                        _ => "trace",
                    }
                    .parse()
                    .unwrap(),
                )
            }),
        )
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .without_time()
        .finish()
        .init();

    let server = HttpServer::new({
        let webroot = match std::fs::canonicalize(resolve_webroot(&opts.webroot)?) {
            Ok(path) => {
                info!(path = %path.display(), "resolved webroot");
                Some(path)
            }
            Err(error) => {
                warn!(%error, "could not resolve webroot");
                None
            }
        };

        move || {
            let app = App::new()
                .wrap(tracing_actix_web::TracingLogger)
                .service(web::scope("/api/v1").service(index));

            if let Some(webroot) = &webroot {
                app.service(actix_files::Files::new("/", &webroot).index_file("index.html"))
            } else {
                app
            }
        }
    })
    .bind(&opts.bind)?;

    info!(bind = opts.bind.as_str(), "social-todo-server running");

    server.run().await
}

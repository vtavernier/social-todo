#[macro_use]
extern crate tracing;

use std::path::PathBuf;

use actix_web::{App, HttpServer};
use structopt::StructOpt;

use tracing_subscriber::util::SubscriberInitExt;

mod api;
mod models;

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

    /// Skip loading .env
    #[structopt(long)]
    no_env: bool,
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

async fn serve_file(webroot: PathBuf, target: &str) -> impl actix_web::Responder {
    actix_files::NamedFile::open(webroot.join(target))
}

fn file_service(webroot: &PathBuf, target: &'static str) -> actix_web::Route {
    let webroot = webroot.to_owned();
    actix_web::web::get().to(move || serve_file(webroot.clone(), target))
}

#[actix_web::main]
async fn main() -> color_eyre::eyre::Result<()> {
    // Install eyre handler
    color_eyre::install()?;

    // Load options
    let opts = Opts::from_args();

    // Load environment variables
    let dotenv_path = if !opts.no_env {
        Some(dotenv::dotenv())
    } else {
        None
    };

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

    // Now that we have logging, we can log the dotenv status
    if let Some(dotenv_path) = dotenv_path {
        match dotenv_path {
            Ok(path) => {
                info!(path = %path.display(), "loaded environment from file");
            }
            Err(error) => {
                warn!(%error, "no environment file loaded");
            }
        }
    }

    // Create the database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    info!(%database_url, "connecting to database");

    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&&database_url)
        .await?;

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
                .data(db_pool.clone())
                .wrap(tracing_actix_web::TracingLogger)
                .configure(api::config);

            if let Some(webroot) = &webroot {
                app.route("/users/", file_service(webroot, "users/index.html"))
                    .route(
                        "/users/{id}/",
                        file_service(webroot, "users/_id/index.html"),
                    )
                    .service(actix_files::Files::new("/", &webroot).index_file("index.html"))
            } else {
                app
            }
        }
    })
    .bind(&opts.bind)?;

    info!(bind = opts.bind.as_str(), "social-todo-server running");

    server.run().await?;

    Ok(())
}

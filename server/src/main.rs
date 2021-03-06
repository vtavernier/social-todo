#[macro_use]
extern crate tracing;

use std::path::PathBuf;

use actix_web::{App, HttpServer};
use structopt::StructOpt;
use tracing_subscriber::util::SubscriberInitExt;

use social_todo_server::models;

mod api;

#[derive(StructOpt)]
struct Opts {
    /// Increase logging verbosity
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u32,

    /// Bind address
    #[structopt(short, long, default_value = "http://127.0.0.1:8880")]
    bind: url::Url,

    /// Path to static files to serve as root
    #[structopt(short, long)]
    webroot: Option<PathBuf>,

    /// Skip loading .env
    #[structopt(long)]
    no_env: bool,

    /// Target database URL
    #[structopt(long, env = "DATABASE_URL")]
    database_url: String,

    /// Session key
    #[structopt(long, env = "SOCIAL_TODO_SESSION_KEY", hide_env_values = true)]
    session_key: String,

    /// Redis URL
    #[structopt(long, env = "REDIS_URL")]
    redis_url: Option<String>,
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

async fn run(opts: &Opts) -> color_eyre::eyre::Result<()> {
    // Resolve the bind url
    let mut bind_url = opts.bind.clone();

    // Do we have $PORT set?
    let has_port = if let Ok(Ok(port)) = std::env::var("PORT").map(|port| port.parse::<u16>()) {
        bind_url.set_host(Some("0.0.0.0"))?;
        bind_url.set_port(Some(port)).unwrap();
        true
    } else {
        false
    };

    // Create the database connection pool
    let database_url = &opts.database_url;
    info!(%database_url, "connecting to database");

    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .connect(database_url)
        .await?;

    // Create the Redis pool
    let redis_pool = if let Some(redis_url) = &opts.redis_url {
        info!(%redis_url, "connecting to Redis");
        let client = redis::Client::open(redis_url.as_str())?;

        // Ping the Redis instance
        let id = std::process::id();
        let result = match client.get_async_connection().await {
            Ok(mut conn) => {
                redis::pipe()
                    .atomic()
                    .set("ping", id)
                    .ignore()
                    .get("ping")
                    .del("ping")
                    .ignore()
                    .query_async::<_, Vec<u32>>(&mut conn)
                    .await
            }
            Err(error) => Err(error),
        };

        let expected: Result<_, redis::RedisError> = Ok(vec![id]);
        if result != expected {
            error!(?expected, actual = ?result, "Redis ping failed, disabling cache");
            None
        } else {
            Some(client.get_multiplexed_async_connection().await?)
        }
    } else {
        info!("no Redis URL specified, disabling cache");
        None
    };

    // Should the cookies have the secure flag?
    //   If PORT is set for the binding, then we have to be running in production
    let cookie_secure = has_port || bind_url.scheme() == "https";

    // Resolve bind url
    let socket_addrs = bind_url.socket_addrs(|| None)?;

    // Create the session middleware
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

        let session_key = opts.session_key.as_bytes().to_vec();

        move || {
            let app = App::new()
                .data(models::Connector {
                    pg_pool: db_pool.clone(),
                    redis_pool: redis_pool.clone(),
                })
                .wrap(
                    // TODO: Session expiration time?
                    actix_session::CookieSession::private(&session_key)
                        .name("social-todo-session")
                        .secure(cookie_secure)
                        .lazy(true)
                        .http_only(true),
                )
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
    .bind(&*socket_addrs)?;

    info!(bind = ?socket_addrs, "social-todo-server running");

    server.run().await?;

    Ok(())
}

fn main() -> color_eyre::eyre::Result<()> {
    // Install eyre handler
    color_eyre::install()?;

    // Load environment variables
    let delimiter: std::ffi::OsString = "--".into();
    let no_env: std::ffi::OsString = "--no-env".into();
    let (raw_no_env, dotenv_path) = if !std::env::args_os()
        .take_while(|arg| *arg != delimiter)
        .any(|arg| arg == no_env)
    {
        (false, Some(dotenv::dotenv()))
    } else {
        (true, None)
    };

    // Load options
    let opts = Opts::from_args();

    // Check that we parsed --no-env correctly
    assert_eq!(opts.no_env, raw_no_env);

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

    actix_rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads((num_cpus::get() / 8).max(4).min(2))
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(run(&opts))
}

use crate::models::Connector;

pub async fn get_connector() -> Connector {
    Connector {
        pg_pool: sqlx::postgres::PgPoolOptions::new()
            .connect(
                std::env::var("DATABASE_URL")
                    .expect("missing DATABASE_URL")
                    .as_str(),
            )
            .await
            .expect("creating pool failed"),
        redis_pool: None,
    }
}

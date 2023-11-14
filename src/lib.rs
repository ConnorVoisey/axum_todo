pub mod config;
pub mod error;
pub mod routes;
use config::Config;
use routes::create_routes;
use sqlx::{migrate::MigrateError, postgres::PgPoolOptions, Pool};
use std::{net::SocketAddr, sync::Arc};

pub async fn connect(config: &Config) -> Pool<sqlx::Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(config.public.db_connections)
        .connect(&config.private.db_url)
        .await
        .expect("Failed to connect to database at provided uri");

    run_migrations(&pool)
        .await
        .expect("unable to run migrations");

    pool
}

async fn run_migrations(pool: &Pool<sqlx::Postgres>) -> Result<(), MigrateError> {
    sqlx::migrate!().run(pool).await
}

pub async fn run() {
    let config = Arc::new(Config::init());
    let pool = connect(&config).await;
    let app = create_routes(pool, config.clone());

    let port = config.clone().public.port;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // serve the api
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

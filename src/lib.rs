pub mod config;
pub mod models;
pub mod error;
pub mod open_api;
pub mod routes;
use config::Config;
use routes::create_routes;
use sqlx::{migrate::MigrateError, postgres::PgPoolOptions, Pool};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;
use axum_tracing_opentelemetry::opentelemetry_tracing_layer;
use crate::config::log::init_tracing;

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
    // get the configuration info
    let config = Arc::new(Config::init());

    // init tracing so that the logs go somewhere
    init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers().expect("failed to init tracing");
    // init_tracing(&config);
    info!("starting application");

    let pool = connect(&config).await;
    let app = create_routes(pool, config.clone());

    let port = config.clone().public.port;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // serve the api
    info!("hosting server at 127.0.0.1:{port}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

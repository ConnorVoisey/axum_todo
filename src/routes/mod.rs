mod todos;
use std::sync::Arc;

use axum::{extract::FromRef, routing::get, Router};

use sqlx::Pool;
use tower_http::cors::CorsLayer;

use crate::config::Config;

use self::todos::todo_router;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database_pool: Pool<sqlx::Postgres>,
    pub config: Arc<Config>,
}

pub fn create_routes(database_pool: Pool<sqlx::Postgres>, config: Arc<Config>) -> Router {
    let app_state = AppState {
        database_pool,
        config,
    };
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/todo", todo_router())
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}

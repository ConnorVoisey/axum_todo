mod todos;
use axum::{extract::FromRef, routing::get, Router};

use sqlx::Pool;
use tower_http::cors::CorsLayer;

use self::todos::{delete, index, show, store, todo_router, update};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database_pool: Pool<sqlx::Postgres>,
}

pub fn create_routes(database_pool: Pool<sqlx::Postgres>) -> Router {
    let app_state = AppState { database_pool };
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/todo", todo_router())
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}

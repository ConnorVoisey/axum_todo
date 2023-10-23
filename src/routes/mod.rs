mod todos;
use axum::{extract::FromRef, routing::get, Router};

use sqlx::Pool;
use tower_http::cors::CorsLayer;

use self::todos::{todo_router, delete, index, show, store, update};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database_pool: Pool<sqlx::Postgres>,
}

pub fn create_routes(database_pool: Pool<sqlx::Postgres>) -> Router {
    let app_state = AppState { database_pool };
    Router::new()
        .route("/", get(|| async { "Hello, Edited World!" }))
        .route("/todo", get(index).post(store))
        .route("/todo/:id", get(show).put(update).delete(delete))
        .layer(CorsLayer::permissive())
        .with_state(app_state)
        .nest("/category", todo_router())
}

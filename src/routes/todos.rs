use crate::error::Result;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use fake::faker::boolean::raw::Boolean;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{FromRow, Pool};
use uuid::Uuid;

use super::AppState;

pub fn todo_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index).post(store))
        .route("/:id", get(show).put(update).delete(delete))
}

#[derive(Serialize, FromRow)]
pub struct Todo {
    id: Uuid,
    title: String,
    description: String,
    completed: bool,
}

pub async fn index(State(pool): State<Pool<sqlx::Postgres>>) -> Result<Json<Vec<Todo>>> {
    Ok(Json(
        sqlx::query_as!(Todo, "SELECT * FROM todo;")
            .fetch_all(&pool)
            .await?,
    ))
}

pub async fn show(
    State(pool): State<Pool<sqlx::Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>> {
    Ok(Json(
        sqlx::query_as!(Todo, "SELECT * FROM todo WHERE id = $1;", id)
            .fetch_one(&pool)
            .await?,
    ))
}

#[derive(Deserialize)]
pub struct TodoInput {
    title: String,
    description: String,
    completed: Option<bool>,
}
pub async fn store(
    State(pool): State<Pool<sqlx::Postgres>>,
    Json(todo_input): Json<TodoInput>,
) -> Result<Json<Todo>> {
    Ok(Json(
        sqlx::query_as!(
            Todo,
            "INSERT INTO todo (title, description, completed) VALUES ($1, $2, $3) RETURNING *;",
            todo_input.title,
            todo_input.description,
            todo_input.completed.unwrap_or_default()
        )
        .fetch_one(&pool)
        .await?,
    ))
}

pub async fn update(
    State(pool): State<Pool<sqlx::Postgres>>,
    Path(id): Path<Uuid>,
    Json(todo_input): Json<TodoInput>,
) -> Result<Json<Todo>> {
    Ok(Json(
        sqlx::query_as!(
            Todo,
            "UPDATE todo SET title = $1, description = $2, completed = $3 WHERE id = $4 RETURNING *;",
            todo_input.title,
            todo_input.description,
            todo_input.completed.unwrap_or_default(),
            id
        )
        .fetch_one(&pool)
        .await?,
    ))
}

#[derive(Deserialize, Serialize)]
pub struct RowsAffected {
    rows_affected: u64,
}
pub async fn delete(
    State(pool): State<Pool<sqlx::Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<Json<RowsAffected>> {
    let res = sqlx::query!("DELETE FROM todo WHERE id = $1", id)
        .execute(&pool)
        .await?;
    Ok(Json(RowsAffected {
        rows_affected: res.rows_affected(),
    }))
}

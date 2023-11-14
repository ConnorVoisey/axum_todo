use super::{AppState, StateAppState};
use crate::error::Error;
use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

pub fn todo_router(state: Arc<AppState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(index, |op| {
                op.description("list all todos")
                    .response::<200, Json<Vec<Todo>>>()
            })
            .post_with(store, |op| {
                op.description("store a todo").response::<201, Json<Todo>>()
            }),
        )
        .api_route(
            "/:id",
            get_with(show, |op| {
                op.description("show a todo from the id")
                    .response::<200, Json<Todo>>()
            })
            .put_with(update, |op| {
                op.description("update a todo from the id")
                    .response::<200, Json<Todo>>()
            })
            .delete_with(delete, |op| {
                op.description("Delete a todo from the id")
                    .response::<200, Json<RowsAffected>>()
            }),
        )
        .with_state(state)
}

#[derive(Serialize, FromRow, JsonSchema)]
pub struct Todo {
    id: Uuid,
    title: String,
    description: String,
    completed: bool,
}

pub async fn index(State(state): StateAppState) -> impl IntoApiResponse {
    let res = sqlx::query_as!(Todo, "SELECT * FROM todo;")
        .fetch_all(&state.pool)
        .await;

    match res {
        Ok(val) => (StatusCode::OK, Json(val)).into_response(),
        Err(err) => Error::Sqlx(err).into_response(),
    }
}

pub async fn show(State(state): StateAppState, Path(id): Path<Uuid>) -> impl IntoApiResponse {
    let res = sqlx::query_as!(Todo, "SELECT * FROM todo WHERE id = $1;", id)
        .fetch_one(&state.pool)
        .await;
    match res {
        Ok(val) => (StatusCode::OK, Json(val)).into_response(),
        Err(err) => Error::Sqlx(err).into_response(),
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct TodoInput {
    title: String,
    description: String,
    completed: Option<bool>,
}
pub async fn store(
    State(state): StateAppState,
    Json(todo_input): Json<TodoInput>,
) -> impl IntoApiResponse {
    let res = sqlx::query_as!(
        Todo,
        "INSERT INTO todo (title, description, completed) VALUES ($1, $2, $3) RETURNING *;",
        todo_input.title,
        todo_input.description,
        todo_input.completed.unwrap_or_default()
    )
    .fetch_one(&state.pool)
    .await;
    match res {
        Ok(val) => (StatusCode::CREATED, Json(val)).into_response(),
        Err(err) => Error::Sqlx(err).into_response(),
    }
}

pub async fn update(
    State(state): StateAppState,
    Path(id): Path<Uuid>,
    Json(todo_input): Json<TodoInput>,
) -> impl IntoApiResponse {
    let res = sqlx::query_as!(
        Todo,
        "UPDATE todo SET title = $1, description = $2, completed = $3 WHERE id = $4 RETURNING *;",
        todo_input.title,
        todo_input.description,
        todo_input.completed.unwrap_or_default(),
        id
    )
    .fetch_one(&state.pool)
    .await;
    match res {
        Ok(val) => (StatusCode::OK, Json(val)).into_response(),
        Err(err) => Error::Sqlx(err).into_response(),
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct RowsAffected {
    rows_affected: u64,
}
pub async fn delete(State(state): StateAppState, Path(id): Path<Uuid>) -> impl IntoApiResponse {
    let res = sqlx::query!("DELETE FROM todo WHERE id = $1", id)
        .execute(&state.pool)
        .await;
    match res {
        Ok(val) => (
            StatusCode::OK,
            Json(RowsAffected {
                rows_affected: val.rows_affected(),
            }),
        )
            .into_response(),
        Err(err) => Error::Sqlx(err).into_response(),
    }
}

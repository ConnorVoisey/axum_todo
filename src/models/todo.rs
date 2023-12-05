use schemars::JsonSchema;
use serde::Serialize;
use sqlx::{FromRow, Pool, Postgres};
use tracing::instrument;
use uuid::Uuid;

#[derive(Serialize, FromRow, JsonSchema)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Todo {
    #[instrument(skip_all)]
    pub async fn index(pool: &Pool<Postgres>) -> Result<Vec<Todo>, sqlx::Error> {
        Ok(sqlx::query_as!(Todo, "SELECT * FROM todo;")
            .fetch_all(pool)
            .await?)
    }
}

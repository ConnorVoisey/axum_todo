use crate::query_params::IndexQueryParams;
use schemars::JsonSchema;
use sea_query::{Alias, Expr, Iden, Order, PostgresQueryBuilder, Query};
use sea_query_binder::{SqlxBinder, SqlxValues};
use serde::{Deserialize, Serialize};
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
#[derive(Debug, Iden, Clone)]
pub enum TodoSchema {
    #[iden = "todo"]
    Table,
    Id,
    Title,
    Description,
    Completed,
}

impl Todo {
    #[instrument(skip_all)]
    pub async fn index(
        pool: &Pool<Postgres>,
        query_params: &IndexQueryParams,
    ) -> Result<Vec<Todo>, sqlx::Error> {
        let (sql, values) = Self::index_sql(query_params);
        sqlx::query_with(&sql, values).fetch_all(pool).await?;
        Ok(sqlx::query_as!(Todo, "SELECT * FROM todo;")
            .fetch_all(pool)
            .await?)
    }

    fn index_sql(query_params: &IndexQueryParams) -> (String, SqlxValues) {
        let mut query = Query::select();
        query.from(TodoSchema::Table).columns([
            TodoSchema::Id,
            TodoSchema::Title,
            TodoSchema::Description,
            TodoSchema::Completed,
        ]);

        // for (col, dir) in &query_params.sorting {
        //     query.order_by(col.clone(), dir.clone());
        // }
        //
        // for (col, filter) in &query_params.filters {
        //     query.and_where(Expr::col(col.clone()).like(&format!("%{filter}%")));
        // }
        //
        query
            .offset(query_params.offset)
            .limit(query_params.limit)
            .build_sqlx(PostgresQueryBuilder {})
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_index_sql() {
        let query_params = IndexQueryParams::default();
        let (sql, values) = Todo::index_sql(&query_params);
        assert_eq!(
            sql,
            String::from(
                r#"SELECT "id", "title", "description", "completed" FROM "todo" LIMIT $1 OFFSET $2"#
            )
        );
    }
}

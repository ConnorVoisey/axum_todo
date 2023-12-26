use std::collections::HashMap;

use axum::http::Uri;
use sea_query::{Alias, Expr, Iden, Order, PostgresQueryBuilder, Query};
use sea_query_binder::{SqlxBinder, SqlxValues};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use tracing::instrument;

// let uri: Uri = "http://localhost:3000/todo?filter[completed]=false&filter[title]=test&sort[0][title][true],sort[1][completed][false]&limit=50&offset=100".parse().unwrap();
#[derive(Debug, Deserialize)]
struct IndexQueryString {
    sort: Vec<SortField>,
    filter: HashMap<String, String>,
    limit: u64,
    offset: u64,
}

#[derive(Debug, Deserialize)]
struct SortField {
    column: String,
    order: SortOrder,
}

#[derive(Debug, Deserialize)]
enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct IndexQueryParams {
    pub sorting: Vec<(String, Order)>,
    pub filters: Vec<(String, String)>, //TODO: this should not be a string
    pub limit: u64,
    pub offset: u64,
}
impl IndexQueryParams {
    pub fn from_uri(uri: &Uri) -> IndexQueryParams {
        //TODO: write this logic properly
        let temp: axum::extract::Query<IndexQueryString> =
            axum::extract::Query::try_from_uri(uri).unwrap();
        IndexQueryParams {
            sorting: vec![],
            filters: vec![],
            limit: temp.limit,
            offset: temp.offset,
        }
    }
}

impl Default for IndexQueryParams {
    fn default() -> Self {
        IndexQueryParams {
            sorting: vec![],
            filters: vec![],
            limit: 20,
            offset: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::todo::TodoSchema;
    #[test]
    fn index_params_from_uri() {
        let uri: Uri = "http://localhost:3000/todo?filter[completed]=false&filter[title]=test&sort[0][title][true]&sort[1][completed][false]&limit=50&offset=100".parse().unwrap();
        let params = IndexQueryParams::from_uri(&uri);
        // let correct_params = IndexQueryParams {
        //     sorting: vec![
        //         (TodoSchema::Title, Order::Asc),
        //         (TodoSchema::Completed, Order::Desc),
        //     ],
        //     filters: vec![
        //         (TodoSchema::Completed, false.to_string()),
        //         (TodoSchema::Title, String::from("test")),
        //     ],
        //     limit: 50,
        //     offset: 100,
        // };
        //
        // // sorting
        // for (i, (col, dir)) in correct_params.sorting.into_iter().enumerate() {
        //     assert!(std::mem::discriminant(&col) == std::mem::discriminant(&params.sorting[i].0));
        //     assert_eq!(dir, params.sorting[i].1);
        // }
        //
        // // filter
        // for (i, (col, filter)) in correct_params.filters.into_iter().enumerate() {
        //     assert!(std::mem::discriminant(&col) == std::mem::discriminant(&params.filters[i].0));
        //     assert_eq!(filter, params.filters[i].1);
        // }
        // assert_eq!(params.limit, correct_params.limit);
        // assert_eq!(params.offset, correct_params.offset);
    }
}

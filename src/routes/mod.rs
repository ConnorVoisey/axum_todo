mod todos;
use self::todos::todo_router;
use crate::{config::Config, open_api::docs_routes};
use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use axum::{
    extract::{FromRef, State},
    Extension, Router,
};
use axum_tracing_opentelemetry::{opentelemetry_tracing_layer, middleware::{OtelInResponseLayer, OtelAxumLayer}};
use sqlx::Pool;
use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::Arc,
};
use tower_http::cors::CorsLayer;

pub type StateAppState = State<Arc<AppState>>;

#[derive(Clone, FromRef, Debug)]
pub struct AppState {
    pub pool: Pool<sqlx::Postgres>,
    pub config: Arc<Config>,
}

pub fn create_routes(pool: Pool<sqlx::Postgres>, config: Arc<Config>) -> Router {
    aide::gen::on_error(|error| {
        println!("{error}");
    });

    aide::gen::extract_schemas(true);
    let mut api = aide::openapi::OpenApi::default();

    let app_state = Arc::new(AppState { pool, config });
    let router = ApiRouter::new()
        .nest_api_service("/todo", todo_router(app_state.clone()))
        .nest_api_service("/docs", docs_routes())
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api.clone())))
        .layer(CorsLayer::permissive()) // include trace context as header into the response
        .layer(OtelInResponseLayer::default())
        //start OpenTelemetry trace on incoming request
        .layer(OtelAxumLayer::default());

    write_open_api(&api);
    router
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Todo Api")
        .summary("An example Todo application")
        .description(include_str!("../../README.md"))
}

fn write_open_api(api: &OpenApi) {
    let file = File::create("./openApi.json").unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &api).unwrap();
    writer.flush().unwrap();
}

pub mod error;
pub mod routes;
use dotenvy::dotenv;
use routes::create_routes;
use sqlx::{migrate::MigrateError, postgres::PgPoolOptions, Pool};

pub async fn connect() -> Pool<sqlx::Postgres> {
    dotenv().ok();
    let uri_string =
        dotenvy::var("DATABASE_URL").expect("Failed to read env variable 'DATABASE_URL'");

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&uri_string)
        .await
        .expect("Failed to connect to database at provided uri");

    run_migrations(&pool)
        .await
        .expect("DB state did not match migrations");

    pool
}

async fn run_migrations(pool: &Pool<sqlx::Postgres>) -> Result<(), MigrateError> {
    sqlx::migrate!().run(pool).await
}

pub async fn run() {
    let app = create_routes(connect().await);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

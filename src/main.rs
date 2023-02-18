use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use serde_json::{json, Value};
use sqlx::mysql::MySqlPoolOptions;

mod error;
mod handlers;
mod models;

async fn not_found_handler() -> Json<Value> {
    Json(json!({
        "message": "Not Found",
    }))
}

async fn root() -> Json<Value> {
    Json(json!({
        "message": "Hello, World!",
    }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env variable");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .fallback(not_found_handler)
        .route("/", get(root))
        .route("/person", post(handlers::person::create))
        .with_state(pool);

    run(app).await
}

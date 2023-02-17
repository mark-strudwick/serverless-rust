use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use serde_json::{json, Value};
use sqlx::{mysql::MySqlPoolOptions, query, query_as, MySql, Pool, Row};

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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct Person {
    name: String,
}

async fn create_person(
    Json(person): Json<Person>,
    State(pool): State<Pool<MySql>>,
) -> Result<Json<Value>, sqlx::Error> {
    let person_id = query!(
        r#"
INSERT INTO person ( name )
VALUES ( ? )
        "#,
        person.name
    )
    .execute(&pool)
    .await?
    .last_insert_id();

    Ok(Json(json!({
        "id": person_id,
        "name": person.name,
    })))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL is not set"))
        .await?;

    let app = Router::new()
        .fallback(not_found_handler)
        .route("/", get(root))
        .route("/person", post(create_person))
        .with_state(pool);

    run(app).await
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

use lambda_http::{
    run,
    Error,
};
use axum::{
    response::Json,
    Router,
    routing::{get, post},
};
use serde_json::{Value, json};

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

async fn person(Json(person): Json<Person>) -> Json<Value> {
    let name = person.name;
    Json(json!({
        "message": format!("Hello, {}!", name),
    }))
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

    let app = Router::new()
        .fallback(not_found_handler)
        .route("/",  get(root))
        .route("/person", post(person));

    run(app).await
}

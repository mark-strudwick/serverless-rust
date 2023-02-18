use axum::{extract::State, response::Json};
use serde_json::{json, Value};
use sqlx::{MySql, Pool};

use crate::{error::AppError, models};

pub async fn create(
    Json(person): Json<models::person::Person>,
    State(pool): State<Pool<MySql>>,
) -> Result<Json<Value>, AppError> {
    if person.name.is_empty() {
        return Err(AppError::BadRequest);
    }

    let result = sqlx::query("INSERT INTO person (name) VALUES (?)")
        .bind(&person.name)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok(Json(json!({
        "name": person.name,
    })))
}

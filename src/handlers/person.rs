use axum::{extract::State, response::Json};
use serde_json::{json, Value};
use sqlx::MySqlPool;

use crate::{error::AppError, models};

pub async fn create(
    State(pool): State<MySqlPool>,
    Json(person): Json<models::person::Person>,
) -> Result<Json<Value>, AppError> {
    if person.name.is_empty() {
        return Err(AppError::BadRequest);
    }

    let result = sqlx::query("INSERT INTO person (name) VALUES (?)")
        .bind(&person.name)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    if result.rows_affected() < 1 {
        Err(AppError::InternalServerError)
    } else {
        Ok(Json(json!({ "name": person.name})))
    }
}

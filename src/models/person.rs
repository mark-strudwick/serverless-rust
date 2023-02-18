use serde::Deserialize;

#[derive(Deserialize, sqlx::FromRow)]
pub struct Person {
    pub name: String,
}

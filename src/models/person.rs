use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, sqlx::FromRow)]
pub struct Person {
    pub name: String,
}

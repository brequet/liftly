use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type, sqlx::FromRow, Debug)]
pub struct ExerciseLight {
    pub id: i32,
    pub predefined_id: Option<String>,
    pub title: String,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type, sqlx::FromRow, Debug, Clone)]
pub struct WorkoutSet {
    pub id: i32,
    pub workout_id: i32,
    pub exercise_id: String,
    pub reps: i32,
    pub weight: f64,
    #[specta(type = String)]
    pub created_at: DateTime<Utc>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::models::workout_set::WorkoutSet;

#[derive(Serialize, Deserialize, Type, sqlx::FromRow, Debug, Clone)]
pub struct Workout {
    pub id: i32,
    #[specta(type = String)]
    pub start_datetime: DateTime<Utc>,
    #[specta(type = String)]
    pub end_datetime: Option<DateTime<Utc>>,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
pub struct WorkoutSession {
    #[serde(flatten)]
    pub workout: Workout,
    pub sets: Vec<WorkoutSet>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;

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

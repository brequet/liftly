use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize, specta::Type)]
#[serde(tag = "type", content = "data")]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("A workout is already in progress.")]
    WorkoutAlreadyInProgress,

    #[error("No active workout found.")]
    NoActiveWorkout,
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Database(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

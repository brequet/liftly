use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, specta::Type)]
#[serde(tag = "type", content = "data")]
pub enum ApiError {
    Database(String),
    WorkoutAlreadyInProgress,
    NoActiveWorkout,
    Internal(String),
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("A workout is already in progress.")]
    WorkoutAlreadyInProgress,

    #[error("No active workout found.")]
    NoActiveWorkout,
}

impl From<DomainError> for ApiError {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::Database(d) => {
                log::error!("A database error occurred: {}", d);
                ApiError::Internal("An unexpected error occurred.".to_string())
            }
            DomainError::WorkoutAlreadyInProgress => ApiError::WorkoutAlreadyInProgress,
            DomainError::NoActiveWorkout => ApiError::NoActiveWorkout,
        }
    }
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;

pub type DomainResult<T> = std::result::Result<T, DomainError>;

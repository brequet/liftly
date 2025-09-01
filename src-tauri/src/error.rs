use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, specta::Type)]
#[serde(tag = "type", content = "data")]
pub enum ApiError {
    Database(String),
    WorkoutAlreadyInProgress,
    NoActiveWorkout,
    SetNotInWorkout { set_id: i32, workout_id: i32 },
    WorkoutSetNotFound { set_id: i32 },
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

    #[error("Set with id {set_id} does not belong to the active workout {workout_id}.")]
    SetNotInWorkout { set_id: i32, workout_id: i32 },

    #[error("The specified workout set with id {set_id} was not found.")]
    WorkoutSetNotFound { set_id: i32 },
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
            DomainError::SetNotInWorkout { set_id, workout_id } => {
                ApiError::SetNotInWorkout { set_id, workout_id }
            }
            DomainError::WorkoutSetNotFound { set_id } => ApiError::WorkoutSetNotFound { set_id },
        }
    }
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;
pub type DomainResult<T> = std::result::Result<T, DomainError>;

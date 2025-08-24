use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::State;

use crate::db::connection::DbPool;
use chrono::{DateTime, Utc};

use super::error::{AppError, Result};

#[derive(Serialize, Deserialize, Type, sqlx::FromRow, Debug)]
pub struct Workout {
    id: i32,
    #[specta(type = String)] // Specta needs a hint to convert DateTime to a string in TS
    start_datetime: DateTime<Utc>,
    #[specta(type = String)]
    end_datetime: Option<DateTime<Utc>>,
    status: String,
    notes: Option<String>,
}

/// Internal helper function to get the currently active workout.
/// An active workout is one with the status 'in_progress'.
async fn _get_active_workout(pool: &DbPool) -> sqlx::Result<Option<Workout>> {
    sqlx::query_as::<_, Workout>("SELECT * FROM workouts WHERE status = 'in_progress' LIMIT 1")
        .fetch_optional(&pool.0)
        .await
}

/// Returns the currently active workout, if one exists.
#[tauri::command]
#[specta::specta]
pub async fn get_active_workout(pool: State<'_, DbPool>) -> Result<Option<Workout>> {
    let workout = _get_active_workout(&pool).await?;
    Ok(workout)
}

/// Creates a new workout session.
///
/// This command will fail with a `WorkoutAlreadyInProgress` error if a workout
/// is already active, ensuring that only one workout can be in progress at a time.
#[tauri::command]
#[specta::specta]
pub async fn create_workout(pool: State<'_, DbPool>) -> Result<Workout> {
    // Check if a workout is already active.
    if _get_active_workout(&pool).await?.is_some() {
        return Err(AppError::WorkoutAlreadyInProgress);
    }

    // No active workout, so create a new one.
    let new_workout = sqlx::query_as::<_, Workout>(
        r#"
        INSERT INTO workouts (start_datetime, status)
        VALUES ($1, 'in_progress')
        RETURNING id, start_datetime, end_datetime, status, notes
        "#,
    )
    .bind(Utc::now())
    .fetch_one(&pool.0)
    .await?;

    Ok(new_workout)
}

/// Ends the currently active workout.
///
/// This command will fail with a `NoActiveWorkout` error if no workout is currently active.
#[tauri::command]
#[specta::specta]
pub async fn end_workout(pool: State<'_, DbPool>) -> Result<Workout> {
    let mut workout = _get_active_workout(&pool)
        .await?
        .ok_or(AppError::NoActiveWorkout)?;

    workout = sqlx::query_as::<_, Workout>(
        r#"
        UPDATE workouts
        SET end_datetime = $1, status = 'completed'
        WHERE id = $2
        RETURNING id, start_datetime, end_datetime, status, notes
        "#,
    )
    .bind(Utc::now())
    .bind(workout.id)
    .fetch_one(&pool.0)
    .await?;

    Ok(workout)
}

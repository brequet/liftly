use crate::domain::AppState;
use crate::error::ApiResult;
use crate::models::workout::{Workout, WorkoutSession};
use crate::models::workout_set::WorkoutSet;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn get_active_workout(state: State<'_, AppState>) -> ApiResult<Option<WorkoutSession>> {
    let workout = state.workout_service.get_active_workout().await?;
    Ok(workout)
}

#[tauri::command]
#[specta::specta]
pub async fn create_workout(state: State<'_, AppState>) -> ApiResult<WorkoutSession> {
    let workout = state.workout_service.create_workout().await?;
    Ok(workout)
}

#[tauri::command]
#[specta::specta]
pub async fn end_workout(state: State<'_, AppState>) -> ApiResult<Workout> {
    let workout = state.workout_service.end_workout().await?;
    Ok(workout)
}

#[tauri::command]
#[specta::specta]
pub async fn add_set_to_active_workout(
    state: State<'_, AppState>,
    exercise_id: i32,
    reps: i32,
    weight: f64,
) -> ApiResult<WorkoutSet> {
    let new_set = state
        .workout_service
        .add_set_to_active_workout(exercise_id, reps, weight)
        .await?;
    Ok(new_set)
}

#[tauri::command]
#[specta::specta]
pub async fn update_set_from_active_workout(
    state: State<'_, AppState>,
    set_id: i32,
    exercise_id: i32,
    reps: i32,
    weight: f64,
) -> ApiResult<WorkoutSet> {
    let new_set = state
        .workout_service
        .update_set_from_active_workout(set_id, exercise_id, reps, weight)
        .await?;
    Ok(new_set)
}

#[tauri::command]
#[specta::specta]
pub async fn remove_set_from_active_workout(
    state: State<'_, AppState>,
    set_id: i32,
) -> ApiResult<()> {
    state
        .workout_service
        .remove_set_from_active_workout(set_id)
        .await?;
    Ok(())
}

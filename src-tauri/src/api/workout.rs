use crate::domain::AppState;
use crate::error::ApiResult;
use crate::models::workout::Workout;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn get_active_workout(state: State<'_, AppState>) -> ApiResult<Option<Workout>> {
    let workout = state.workout_service.get_active_workout().await?;
    Ok(workout)
}

#[tauri::command]
#[specta::specta]
pub async fn create_workout(state: State<'_, AppState>) -> ApiResult<Workout> {
    let workout = state.workout_service.create_workout().await?;
    Ok(workout)
}

#[tauri::command]
#[specta::specta]
pub async fn end_workout(state: State<'_, AppState>) -> ApiResult<Workout> {
    let workout = state.workout_service.end_workout().await?;
    Ok(workout)
}

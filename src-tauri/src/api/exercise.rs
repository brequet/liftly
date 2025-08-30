use crate::domain::AppState;
use crate::error::ApiResult;
use crate::models::exercise::ExerciseLight;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn search_exercises(
    state: State<'_, AppState>,
    query: String,
) -> ApiResult<Vec<ExerciseLight>> {
    let exercises = state.exercise_service.search_exercises(query).await?;
    Ok(exercises)
}

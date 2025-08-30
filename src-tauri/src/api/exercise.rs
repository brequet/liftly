use crate::domain::AppState;
use crate::error::ApiResult;
use crate::models::exercise::ExerciseLight;
use crate::models::pagination::{Paginated, PaginationParams};
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn search_exercises(
    state: State<'_, AppState>,
    query: String,
    pagination: PaginationParams,
) -> ApiResult<Paginated<ExerciseLight>> {
    let exercises = state
        .exercise_service
        .search_exercises(query, pagination)
        .await?;
    Ok(exercises)
}

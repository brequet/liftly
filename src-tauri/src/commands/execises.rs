use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::State;

use super::error::Result;
use crate::db::connection::DbPool;

#[derive(Serialize, Deserialize, Type, sqlx::FromRow, Debug)]
pub struct ExerciseLight {
    id: i32,
    predefined_id: Option<String>,
    title: String,
}

#[tauri::command]
#[specta::specta]
pub async fn search_exercises(
    pool: State<'_, DbPool>,
    mut query: String,
) -> Result<Vec<ExerciseLight>> {
    if query.is_empty() {
        let exercises = sqlx::query_as::<_, ExerciseLight>(
            r#"
            SELECT id, predefined_id, title
            FROM exercises
            ORDER BY id
            "#,
        )
        .fetch_all(&pool.0)
        .await?;
        return Ok(exercises);
    }

    query.push('*');

    let exercises = sqlx::query_as::<_, ExerciseLight>(
        r#"
       SELECT
            e.id,
            e.predefined_id,
            e.title
        FROM exercises AS e
        JOIN exercises_fts AS fts ON e.id = fts.rowid
        WHERE fts.exercises_fts MATCH $1
        ORDER BY fts.rank
        "#,
    )
    .bind(query)
    .fetch_all(&pool.0)
    .await?;

    Ok(exercises)
}

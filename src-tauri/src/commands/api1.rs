use tauri::State;

use crate::db::connection::DbPool;

#[tauri::command]
#[specta::specta]
pub fn hello_world(my_name: String) -> String {
    format!("Hello, {my_name}! You've been greeted from Rust!")
}

#[tauri::command]
#[specta::specta]
pub fn goodbye_world() -> String {
    "Goodbye!".to_string()
}

#[tauri::command]
#[specta::specta]
pub async fn get_db_tables(pool: State<'_, DbPool>) -> Result<Vec<String>, String> {
    let query = "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'";

    let table_names = sqlx::query_as::<_, (String,)>(query)
        .fetch_all(&pool.0)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|(name,)| name)
        .collect();

    Ok(table_names)
}

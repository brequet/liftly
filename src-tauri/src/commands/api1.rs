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

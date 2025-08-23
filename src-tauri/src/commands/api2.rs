use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type)]
pub struct MyStruct {
    a: String,
}

#[derive(Serialize, Deserialize, Type)]
pub struct MyResponse {
    message: String,
}

#[tauri::command]
#[specta::specta]
pub fn another_command(data: MyStruct) -> Result<MyResponse, String> {
    Ok(MyResponse {
        message: format!("Received: {}", data.a),
    })
}

#[tauri::command]
#[specta::specta]
pub fn print_log() {
    println!("Log message");
}

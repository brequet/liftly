use tauri::Manager;

pub mod db;

pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = commands::specta_builder();

    let (db_plugin_url, migrations) = db::connection::get_db_plugin_config();

    let mut log_builder = tauri_plugin_log::Builder::new();

    if cfg!(debug_assertions) {
        log_builder = log_builder.level(log::LevelFilter::Info);
    }

    tauri::Builder::default()
        .plugin(log_builder.build())
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations(&db_plugin_url, migrations)
                .build(),
        )
        .setup(|app| {
            let pool = tauri::async_runtime::block_on(db::connection::init_db_pool(app.handle()))?;
            app.manage(pool);
            // builder.mount_events(app); //TODO for events
            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

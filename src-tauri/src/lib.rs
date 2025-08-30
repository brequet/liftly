use tauri::Manager;

pub mod domain;
pub mod error;
pub mod models;
pub mod repository;

pub mod db;

pub mod api;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = api::specta_builder();
    let (db_plugin_url, migrations) = db::connection::get_db_plugin_config();
    let mut log_builder = tauri_plugin_log::Builder::new();

    log_builder = log_builder.level(log::LevelFilter::Info); // TODO: log level ?

    tauri::Builder::default()
        .plugin(log_builder.build())
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations(&db_plugin_url, migrations)
                .build(),
        )
        .setup(|app| {
            let pool = tauri::async_runtime::block_on(db::connection::init_db_pool(app.handle()))?;

            tauri::async_runtime::block_on(db::seed::seed_exercises_if_needed(&pool.0))?;

            let app_state = domain::AppState::new(pool);
            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

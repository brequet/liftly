use once_cell::sync::Lazy;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_sql::{Migration, MigrationKind};

const DB_NAME: &str = "liftly.db";
static DB_URL: Lazy<String> = Lazy::new(|| format!("sqlite:{}", DB_NAME));

pub struct DbPool(pub Pool<Sqlite>);

fn get_app_data_dir(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("Failed to get app data directory")
}

pub async fn init_db_pool(app: &AppHandle) -> Result<DbPool, sqlx::Error> {
    let app_data_dir = get_app_data_dir(app);
    let db_path = app_data_dir.join(DB_NAME);

    if !db_path.exists() {
        std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
        std::fs::File::create(&db_path).expect("Failed to create database file");
    }

    log::info!("Database path: {:?}", db_path);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path.to_str().unwrap())
        .await?;
    Ok(DbPool(pool))
}

pub fn get_db_plugin_config() -> (String, Vec<Migration>) {
    (DB_URL.to_string(), get_migrations())
}

fn get_migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "create_users_table",
        sql: "CREATE TABLE IF NOT EXISTS users (
                  id    INTEGER PRIMARY KEY AUTOINCREMENT,
                  name  TEXT NOT NULL,
                  email TEXT UNIQUE NOT NULL
              )",
        kind: MigrationKind::Up,
    }]
}

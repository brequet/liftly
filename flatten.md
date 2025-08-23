# Flattened Codebase

Total files: 12

## Table of Contents

1. [.\src-tauri\Cargo.toml](#file-1)
2. [.\src-tauri\build.rs](#file-2)
3. [.\src-tauri\capabilities\default.json](#file-3)
4. [.\src-tauri\src\bin\export_bindings.rs](#file-4)
5. [.\src-tauri\src\commands\api1.rs](#file-5)
6. [.\src-tauri\src\commands\api2.rs](#file-6)
7. [.\src-tauri\src\commands\mod.rs](#file-7)
8. [.\src-tauri\src\db\connection.rs](#file-8)
9. [.\src-tauri\src\db\mod.rs](#file-9)
10. [.\src-tauri\src\lib.rs](#file-10)
11. [.\src-tauri\src\main.rs](#file-11)
12. [.\src-tauri\tauri.conf.json](#file-12)

## File 1: .\src-tauri\Cargo.toml

```toml
[package]
name = "liftly"
version = "0.1.0"
description = "Liftly App"
authors = ["brequet"]
license = ""
repository = ""
edition = "2021"
rust-version = "1.77.2"
default-run = "liftly"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
name = "app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[[bin]]
name = "export-bindings"
path = "src/bin/export_bindings.rs"

[build-dependencies]
tauri-build = { version = "2.3.1", features = [] }

[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
log = "0.4"
tauri = { version = "2.7.0", features = [] }
tauri-plugin-log = "2"
specta = "=2.0.0-rc.22"
tauri-specta = { version = "=2.0.0-rc.21", features = ["derive", "typescript"] }
specta-typescript = "0.0.9"
tauri-plugin-sql = { version = "2", features = ["sqlite"] }
sqlx = { version = "0.8.6", features = ["sqlite"] }
once_cell = "1.21.3"
```

## File 2: .\src-tauri\build.rs

```rs
fn main() {
    tauri_build::build();
}
```

## File 3: .\src-tauri\capabilities\default.json

```json
{
	"$schema": "../gen/schemas/desktop-schema.json",
	"identifier": "default",
	"description": "enables the default permissions",
	"windows": ["main"],
	"permissions": ["core:default", "sql:default", "sql:allow-execute"]
}
```

## File 4: .\src-tauri\src\bin\export_bindings.rs

```rs
use specta_typescript::Typescript;

fn main() {
    let output_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./src/lib/bindings.ts".to_string());

    let builder = app_lib::commands::specta_builder();

    println!("Generating TypeScript bindings to: {}", output_path);

    builder
        .export(Typescript::default(), &output_path)
        .expect("Failed to export typescript bindings");

    println!("✅ TypeScript bindings generated successfully!");
}
```

## File 5: .\src-tauri\src\commands\api1.rs

```rs
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
```

## File 6: .\src-tauri\src\commands\api2.rs

```rs
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
```

## File 7: .\src-tauri\src\commands\mod.rs

```rs
use tauri_specta::{collect_commands, Builder, Commands};

mod api1;
mod api2;

macro_rules! combine_commands {
    ( $( $module:ident : [ $( $command:ident ),* ] ),* ) => {
        collect_commands![
            $( $( $module::$command ),* ),*
        ]
    };
}

fn get_all_commands() -> Commands<tauri::Wry> {
    combine_commands!(
        api1: [
            hello_world,
            goodbye_world,
            get_db_tables
        ],
        api2: [
            another_command,
            print_log
        ]
    )
}

pub fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(get_all_commands())
}
```

## File 8: .\src-tauri\src\db\connection.rs

```rs
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
```

## File 9: .\src-tauri\src\db\mod.rs

```rs
pub mod connection;
```

## File 10: .\src-tauri\src\lib.rs

```rs
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
```

## File 11: .\src-tauri\src\main.rs

```rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    app_lib::run()
}
```

## File 12: .\src-tauri\tauri.conf.json

```json
{
	"$schema": "../node_modules/@tauri-apps/cli/config.schema.json",
	"productName": "liftly",
	"version": "0.1.0",
	"identifier": "fr.requet.liftly",
	"build": {
		"frontendDist": "../build",
    	"devUrl": "http://localhost:1420",
		"beforeDevCommand": "pnpm dev",
		"beforeBuildCommand": "pnpm build"
	},
	"app": {
		"windows": [
			{
				"title": "liftly",
				"width": 800,
				"height": 600,
				"resizable": true,
				"fullscreen": false
			}
		],
		"security": {
			"csp": null
		}
	},
	"bundle": {
		"active": true,
		"targets": "all",
		"icon": [
			"icons/32x32.png",
			"icons/128x128.png",
			"icons/128x128@2x.png",
			"icons/icon.icns",
			"icons/icon.ico"
		]
	}
}
```


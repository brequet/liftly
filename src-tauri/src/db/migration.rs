use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "init",
        sql: include_str!("./resources/sql/0001_init.sql"),
        kind: MigrationKind::Up,
    }]
}

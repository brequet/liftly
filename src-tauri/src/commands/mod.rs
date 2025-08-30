use tauri_specta::{collect_commands, Builder, Commands};

mod execises;
mod workout;

mod error;

macro_rules! combine_commands {
    ( $( $module:ident : [ $( $command:ident ),* ] ),* ) => {
        collect_commands![
            $( $( $module::$command ),* ),*
        ]
    };
}

fn get_all_commands() -> Commands<tauri::Wry> {
    combine_commands!(
        execises: [
            search_exercises
        ],
        workout: [
            get_active_workout,
            create_workout,
            end_workout
        ]
    )
}

pub fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(get_all_commands())
}

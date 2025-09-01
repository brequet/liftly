use tauri_specta::{collect_commands, Builder, Commands};

mod exercise;
mod workout;

macro_rules! combine_commands {
    ( $( $module:ident : [ $( $command:ident ),* ] ),* ) => {
        collect_commands![
            $( $( $module::$command ),* ),*
        ]
    };
}

fn get_all_commands() -> Commands<tauri::Wry> {
    combine_commands!(
        exercise: [
            search_exercises
        ],
        workout: [
            get_active_workout,
            create_workout,
            end_workout,
            add_set_to_active_workout,
            update_set_from_active_workout,
            remove_set_from_active_workout
        ]
    )
}

pub fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(get_all_commands())
}

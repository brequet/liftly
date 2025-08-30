use crate::db::connection::DbPool;
use crate::repository::{exercise::ExerciseRepository, workout::WorkoutRepository, Repository};

pub mod exercise;
pub mod workout;

#[derive(Clone)]
pub struct AppState {
    pub exercise_service: exercise::ExerciseService,
    pub workout_service: workout::WorkoutService,
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        let exercise_repo = ExerciseRepository::new(pool.clone());
        let workout_repo = WorkoutRepository::new(pool);

        Self {
            exercise_service: exercise::ExerciseService::new(exercise_repo),
            workout_service: workout::WorkoutService::new(workout_repo),
        }
    }
}

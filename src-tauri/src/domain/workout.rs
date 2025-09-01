use crate::error::{DomainError, DomainResult};
use crate::models::workout::{Workout, WorkoutSession};
use crate::models::workout_set::WorkoutSet;
use crate::repository::workout::WorkoutRepository;
use chrono::Utc;

#[derive(Clone)]
pub struct WorkoutService {
    repo: WorkoutRepository,
}

impl WorkoutService {
    pub fn new(repo: WorkoutRepository) -> Self {
        Self { repo }
    }

    pub async fn get_active_workout(&self) -> DomainResult<Option<WorkoutSession>> {
        let workout = self.repo.find_active_workout_session().await?;
        Ok(workout)
    }

    pub async fn create_workout(&self) -> DomainResult<WorkoutSession> {
        if self.get_active_workout().await?.is_some() {
            return Err(DomainError::WorkoutAlreadyInProgress);
        }

        let new_workout = self.repo.create_workout(Utc::now()).await?;

        let workout_session = WorkoutSession {
            workout: new_workout,
            sets: Vec::new(), // The sets vector is empty
        };

        Ok(workout_session)
    }

    pub async fn end_workout(&self) -> DomainResult<Workout> {
        let active_workout = self
            .get_active_workout()
            .await?
            .ok_or(DomainError::NoActiveWorkout)?;

        let ended_workout = self
            .repo
            .end_workout(active_workout.workout.id, Utc::now())
            .await?;
        Ok(ended_workout)
    }

    pub async fn add_set_to_active_workout(
        &self,
        exercise_id: i32,
        reps: i32,
        weight: f64,
    ) -> DomainResult<WorkoutSet> {
        let active_workout = self
            .get_active_workout()
            .await?
            .ok_or(DomainError::NoActiveWorkout)?;

        let new_set = self
            .repo
            .add_workout_set(active_workout.workout.id, exercise_id, reps, weight)
            .await?;

        Ok(new_set)
    }

    pub async fn update_set_from_active_workout(
        &self,
        set_id: i32,
        exercise_id: i32,
        reps: i32,
        weight: f64,
    ) -> DomainResult<WorkoutSet> {
        let active_workout = self
            .get_active_workout()
            .await?
            .ok_or(DomainError::NoActiveWorkout)?;

        if !active_workout.sets.iter().any(|s| s.id == set_id) {
            return Err(DomainError::SetNotInWorkout {
                set_id,
                workout_id: active_workout.workout.id,
            });
        }

        let updated_set = self
            .repo
            .update_workout_set(set_id, exercise_id, reps, weight)
            .await?;

        Ok(updated_set)
    }

    pub async fn remove_set_from_active_workout(&self, set_id: i32) -> DomainResult<()> {
        let active_workout = self
            .get_active_workout()
            .await?
            .ok_or(DomainError::NoActiveWorkout)?;

        if !active_workout.sets.iter().any(|s| s.id == set_id) {
            return Err(DomainError::SetNotInWorkout {
                set_id,
                workout_id: active_workout.workout.id,
            });
        }

        let rows_affected = self.repo.remove_workout_set(set_id).await?;
        if rows_affected == 0 {
            return Err(DomainError::WorkoutSetNotFound { set_id });
        }

        Ok(())
    }
}

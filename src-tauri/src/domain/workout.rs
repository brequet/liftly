use crate::error::{DomainError, DomainResult};
use crate::models::workout::Workout;
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

    pub async fn get_active_workout(&self) -> DomainResult<Option<Workout>> {
        let workout = self.repo.find_active_workout().await?;
        Ok(workout)
    }

    pub async fn create_workout(&self) -> DomainResult<Workout> {
        if self.get_active_workout().await?.is_some() {
            return Err(DomainError::WorkoutAlreadyInProgress);
        }
        let new_workout = self.repo.create_workout(Utc::now()).await?;
        Ok(new_workout)
    }

    pub async fn end_workout(&self) -> DomainResult<Workout> {
        let active_workout = self
            .get_active_workout()
            .await?
            .ok_or(DomainError::NoActiveWorkout)?;

        let ended_workout = self.repo.end_workout(active_workout.id, Utc::now()).await?;
        Ok(ended_workout)
    }
}

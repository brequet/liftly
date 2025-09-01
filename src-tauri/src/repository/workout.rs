use crate::models::workout::{Workout, WorkoutSession};
use crate::repository::Repository;
use crate::{db::connection::DbPool, models::workout_set::WorkoutSet};
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct WorkoutRepository {
    pool: DbPool,
}

impl Repository for WorkoutRepository {
    fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl WorkoutRepository {
    pub async fn find_active_workout_session(&self) -> sqlx::Result<Option<WorkoutSession>> {
        let workout = sqlx::query_as::<_, Workout>(
            "SELECT * FROM workouts WHERE status = 'in_progress' LIMIT 1",
        )
        .fetch_optional(&self.pool.0)
        .await?;

        if let Some(workout) = workout {
            let sets = sqlx::query_as::<_, WorkoutSet>(
                "SELECT * FROM workout_sets WHERE workout_id = $1 ORDER BY created_at ASC",
            )
            .bind(workout.id)
            .fetch_all(&self.pool.0)
            .await?;

            Ok(Some(WorkoutSession { workout, sets }))
        } else {
            Ok(None)
        }
    }

    pub async fn create_workout(&self, start_time: DateTime<Utc>) -> sqlx::Result<Workout> {
        sqlx::query_as::<_, Workout>(
            r#"
            INSERT INTO workouts (start_datetime, status)
            VALUES ($1, 'in_progress')
            RETURNING id, start_datetime, end_datetime, status, notes
            "#,
        )
        .bind(start_time)
        .fetch_one(&self.pool.0)
        .await
    }

    pub async fn end_workout(&self, id: i32, end_time: DateTime<Utc>) -> sqlx::Result<Workout> {
        sqlx::query_as::<_, Workout>(
            r#"
            UPDATE workouts
            SET end_datetime = $1, status = 'completed'
            WHERE id = $2
            RETURNING id, start_datetime, end_datetime, status, notes
            "#,
        )
        .bind(end_time)
        .bind(id)
        .fetch_one(&self.pool.0)
        .await
    }

    pub async fn add_workout_set(
        &self,
        workout_id: i32,
        exercise_id: i32,
        reps: i32,
        weight: f64,
    ) -> sqlx::Result<WorkoutSet> {
        sqlx::query_as::<_, WorkoutSet>(
            r#"
            INSERT INTO workout_sets (workout_id, exercise_id, reps, weight)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(workout_id)
        .bind(exercise_id)
        .bind(reps)
        .bind(weight)
        .fetch_one(&self.pool.0)
        .await
    }

    pub async fn update_workout_set(
        &self,
        id: i32,
        exercise_id: i32,
        reps: i32,
        weight: f64,
    ) -> sqlx::Result<WorkoutSet> {
        sqlx::query_as::<_, WorkoutSet>(
            r#"
            UPDATE workout_sets
            SET exercise_id = $1, reps = $2, weight = $3
            WHERE id = $4
            RETURNING *
            "#,
        )
        .bind(exercise_id)
        .bind(reps)
        .bind(weight)
        .bind(id)
        .fetch_one(&self.pool.0)
        .await
    }

    pub async fn remove_workout_set(&self, id: i32) -> sqlx::Result<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM workout_sets
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool.0)
        .await?;

        Ok(result.rows_affected())
    }
}

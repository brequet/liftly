use crate::db::connection::DbPool;
use crate::models::workout::Workout;
use crate::repository::Repository;
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
    pub async fn find_active_workout(&self) -> sqlx::Result<Option<Workout>> {
        sqlx::query_as::<_, Workout>("SELECT * FROM workouts WHERE status = 'in_progress' LIMIT 1")
            .fetch_optional(&self.pool.0)
            .await
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
}

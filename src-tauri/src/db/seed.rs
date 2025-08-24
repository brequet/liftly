use serde::Deserialize;
use sqlx::{Executor, SqlitePool};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SeedError {
    #[error("A database error occurred.")]
    Sqlx(#[from] sqlx::Error),

    #[error("Failed to deserialize seed data from JSON.")]
    Serde(#[from] serde_json::Error),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ExerciseSeed {
    id: String,
    name: String,
    title: String,
    #[serde(rename = "primer")]
    description: String,
    #[serde(rename = "type")]
    exercise_type: String,
    primary: Vec<String>,
    secondary: Vec<String>,
    equipment: Vec<String>,
    steps: Vec<String>,
    tips: Vec<String>,
}

async fn insert_exercise<'a, E>(executor: E, exercise: &ExerciseSeed) -> Result<(), SeedError>
where
    E: Executor<'a, Database = sqlx::Sqlite>,
{
    sqlx::query(
        r#"
        INSERT INTO exercises 
        (predefined_id, name, title, description, type, equipment, primary_muscles, secondary_muscles, steps, tips)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&exercise.id)
    .bind(&exercise.name)
    .bind(&exercise.title)
    .bind(&exercise.description)
    .bind(&exercise.exercise_type)
    .bind(serde_json::to_string(&exercise.equipment)?)
    .bind(serde_json::to_string(&exercise.primary)?)
    .bind(serde_json::to_string(&exercise.secondary)?)
    .bind(serde_json::to_string(&exercise.steps)?)
    .bind(serde_json::to_string(&exercise.tips)?)
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn seed_exercises_if_needed(pool: &SqlitePool) -> Result<(), SeedError> {
    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM exercises WHERE predefined_id IS NOT NULL")
            .fetch_one(pool)
            .await?;

    if count.0 > 0 {
        return Ok(());
    }
    log::info!("Seeding predefined exercises...");

    let seed_data = include_str!("./resources/exercises.json");
    let exercises: Vec<ExerciseSeed> = serde_json::from_str(seed_data)?;

    let mut tx = pool.begin().await?;

    for exercise in &exercises {
        insert_exercise(&mut *tx, exercise).await?;
    }

    tx.commit().await?;

    log::info!("Successfully seeded {} exercises.", exercises.len());

    Ok(())
}

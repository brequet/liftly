use crate::db::connection::DbPool;
use crate::models::exercise::ExerciseLight;
use crate::repository::Repository;

#[derive(Clone)]
pub struct ExerciseRepository {
    pool: DbPool,
}

impl Repository for ExerciseRepository {
    fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ExerciseRepository {
    pub async fn search_exercises(&self, query: &str) -> sqlx::Result<Vec<ExerciseLight>> {
        let search_query = format!("{}*", query);

        sqlx::query_as::<_, ExerciseLight>(
            r#"
           SELECT
                e.id,
                e.predefined_id,
                e.title
            FROM exercises AS e
            JOIN exercises_fts AS fts ON e.id = fts.rowid
            WHERE fts.exercises_fts MATCH $1
            ORDER BY fts.rank
            "#,
        )
        .bind(search_query)
        .fetch_all(&self.pool.0)
        .await
    }

    pub async fn get_all_exercises(&self) -> sqlx::Result<Vec<ExerciseLight>> {
        sqlx::query_as::<_, ExerciseLight>(
            r#"
            SELECT id, predefined_id, title
            FROM exercises
            ORDER BY id
            "#,
        )
        .fetch_all(&self.pool.0)
        .await
    }
}

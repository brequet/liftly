use crate::db::connection::DbPool;
use crate::models::exercise::ExerciseLight;
use crate::models::pagination::{Paginated, PaginationParams};
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
    pub async fn search_exercises(
        &self,
        query: &str,
        pagination: PaginationParams,
    ) -> sqlx::Result<Paginated<ExerciseLight>> {
        let search_query = format!("{}*", query);
        let offset = (pagination.page - 1) * pagination.page_size;

        let mut items = sqlx::query_as::<_, ExerciseLight>(
            r#"
           SELECT
                e.id,
                e.predefined_id,
                e.title
            FROM exercises AS e
            JOIN exercises_fts AS fts ON e.id = fts.rowid
            WHERE fts.exercises_fts MATCH $1
            ORDER BY fts.rank
            LIMIT $2
            OFFSET $3
            "#,
        )
        .bind(search_query)
        .bind(pagination.page_size + 1) // Fetch one extra to check if there are more
        .bind(offset)
        .fetch_all(&self.pool.0)
        .await?;

        let has_more = items.len() > pagination.page_size as usize;
        if has_more {
            items.pop();
        }

        Ok(Paginated {
            items,
            has_more,
            page: pagination.page,
            page_size: pagination.page_size,
        })
    }

    pub async fn get_all_exercises(
        &self,
        pagination: PaginationParams,
    ) -> sqlx::Result<Paginated<ExerciseLight>> {
        let offset = (pagination.page - 1) * pagination.page_size;

        let mut items = sqlx::query_as::<_, ExerciseLight>(
            r#"
            SELECT id, predefined_id, title
            FROM exercises
            ORDER BY id
            LIMIT $1
            OFFSET $2
            "#,
        )
        .bind(pagination.page_size + 1) // Fetch one extra to check if there are more
        .bind(offset)
        .fetch_all(&self.pool.0)
        .await?;

        let has_more = items.len() > pagination.page_size as usize;
        if has_more {
            items.pop();
        }

        Ok(Paginated {
            items,
            has_more,
            page: pagination.page,
            page_size: pagination.page_size,
        })
    }
}

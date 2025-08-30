use crate::error::DomainResult;
use crate::models::exercise::ExerciseLight;
use crate::repository::exercise::ExerciseRepository;

#[derive(Clone)]
pub struct ExerciseService {
    repo: ExerciseRepository,
}

impl ExerciseService {
    pub fn new(repo: ExerciseRepository) -> Self {
        Self { repo }
    }

    pub async fn search_exercises(&self, query: String) -> DomainResult<Vec<ExerciseLight>> {
        let exercises = if query.is_empty() {
            self.repo.get_all_exercises().await?
        } else {
            self.repo.search_exercises(&query).await?
        };
        Ok(exercises)
    }
}

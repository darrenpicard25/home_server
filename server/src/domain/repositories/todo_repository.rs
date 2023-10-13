use axum::async_trait;

use crate::domain::entities::todo::Todo;

use super::error::RepositoryResult;

#[derive(Debug)]
pub struct UpdateInput {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct CreateInput {
    pub title: String,
    pub description: String,
}

#[async_trait]
pub trait TodoRepositoryPort: Send + Sync {
    async fn list(&self) -> RepositoryResult<Vec<Todo>>;
    async fn find_by_id(&self, id: i64) -> RepositoryResult<Todo>;
    async fn update_one(&self, input: UpdateInput) -> RepositoryResult<Todo>;
    async fn create(&self, input: CreateInput) -> RepositoryResult<Todo>;
}

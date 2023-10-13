use std::sync::Arc;

use crate::{
    application::use_cases::error::UseCaseResult,
    domain::{entities::todo::Todo, repositories::todo_repository::TodoRepositoryPort},
};

pub struct ListTodosUseCase {
    todo_repository: Arc<dyn TodoRepositoryPort>,
}

impl ListTodosUseCase {
    pub fn new(todo_repository: Arc<dyn TodoRepositoryPort>) -> Self {
        Self { todo_repository }
    }

    pub async fn execute(&self) -> UseCaseResult<Vec<Todo>> {
        let todos = self.todo_repository.list().await?;

        Ok(todos)
    }
}

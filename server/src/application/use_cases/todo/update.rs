use std::sync::Arc;

use crate::{
    application::use_cases::error::UseCaseResult,
    domain::{
        entities::todo::Todo,
        repositories::todo_repository::{TodoRepositoryPort, UpdateInput},
    },
};

pub struct UpdateTodoInput {
    pub title: Option<String>,
    pub description: Option<String>,
}

pub struct UpdateTodoUseCase {
    todo_repository: Arc<dyn TodoRepositoryPort>,
}

impl UpdateTodoUseCase {
    pub fn new(todo_repository: Arc<dyn TodoRepositoryPort>) -> Self {
        Self { todo_repository }
    }

    pub async fn execute(&self, id: i64, input: UpdateTodoInput) -> UseCaseResult<Todo> {
        if input.title.is_none() && input.description.is_none() {
            return Ok(self.todo_repository.find_by_id(id).await?);
        }

        let update_input = UpdateInput {
            id,
            title: input.title,
            description: input.description,
        };

        let todo = self.todo_repository.update_one(update_input).await?;

        Ok(todo)
    }
}

use std::sync::Arc;

use crate::{
    application::use_cases::error::UseCaseResult,
    domain::{
        entities::todo::Todo,
        repositories::todo_repository::{CreateInput, TodoRepositoryPort},
    },
};

pub struct CreateTodoInput {
    pub title: String,
    pub description: String,
}

pub struct CreateTodoUseCase {
    todo_repository: Arc<dyn TodoRepositoryPort>,
}

impl CreateTodoUseCase {
    pub fn new(todo_repository: Arc<dyn TodoRepositoryPort>) -> Self {
        Self { todo_repository }
    }

    pub async fn execute(&self, input: CreateTodoInput) -> UseCaseResult<Todo> {
        let input = CreateInput {
            title: input.title,
            description: input.description,
        };

        let todo = self.todo_repository.create(input).await?;

        Ok(todo)
    }
}

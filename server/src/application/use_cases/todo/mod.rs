use std::sync::Arc;

use crate::domain::repositories::todo_repository::TodoRepositoryPort;

use self::{
    create::CreateTodoUseCase, get::GetTodoUseCase, list::ListTodosUseCase,
    update::UpdateTodoUseCase,
};

pub mod create;
pub mod get;
pub mod list;
pub mod update;

pub struct TodoUseCases {
    pub list: Arc<ListTodosUseCase>,
    pub get: Arc<GetTodoUseCase>,
    pub update: Arc<UpdateTodoUseCase>,
    pub create: Arc<CreateTodoUseCase>,
}

impl TodoUseCases {
    pub fn new(todo_repository: Arc<dyn TodoRepositoryPort>) -> Self {
        Self {
            list: Arc::new(ListTodosUseCase::new(todo_repository.clone())),
            get: Arc::new(GetTodoUseCase::new(todo_repository.clone())),
            update: Arc::new(UpdateTodoUseCase::new(todo_repository.clone())),
            create: Arc::new(CreateTodoUseCase::new(todo_repository)),
        }
    }
}

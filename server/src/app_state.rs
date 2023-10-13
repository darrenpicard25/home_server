use std::sync::Arc;

use axum::extract::FromRef;
use leptos::LeptosOptions;

use crate::{
    application::use_cases::todo::TodoUseCases,
    error::ServiceStartupError,
    infrastructure::{
        repositories::{todo_repository::TodoRepository, user_repository::UserRepository},
        Database,
    },
};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub todo_use_cases: Arc<TodoUseCases>,
}

impl AppState {
    pub async fn new(
        connection_string: &str,
        leptos_options: LeptosOptions,
    ) -> Result<Self, ServiceStartupError> {
        let database = Database::new(connection_string).await?;

        // Repositories
        let todo_repository = Arc::new(TodoRepository::new(database.clone()));
        let _user_repository = Arc::new(UserRepository::new(database));

        // Use Cases
        let todo_use_cases = Arc::new(TodoUseCases::new(todo_repository));

        Ok(Self {
            leptos_options,
            todo_use_cases,
        })
    }
}

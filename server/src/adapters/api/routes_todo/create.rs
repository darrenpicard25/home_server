use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{
    adapters::api::error::ApiResult, app_state::AppState,
    application::use_cases::todo::create::CreateTodoInput,
};

use super::ApiTodo;

#[derive(Debug, Deserialize)]
pub struct CreatePayload {
    title: String,
    description: String,
}

pub async fn handler(
    State(AppState { todo_use_cases, .. }): State<AppState>,
    Json(payload): Json<CreatePayload>,
) -> ApiResult<ApiTodo> {
    log::info!("Post /todo | {payload:?}");

    let input = CreateTodoInput {
        title: payload.title,
        description: payload.description,
    };

    Ok(todo_use_cases.create.execute(input).await?.into())
}

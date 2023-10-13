use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;

use crate::{
    adapters::api::error::ApiResult, app_state::AppState,
    application::use_cases::todo::update::UpdateTodoInput,
};

use super::ApiTodo;

#[derive(Debug, Deserialize)]
pub struct UpdatePayload {
    title: Option<String>,
    description: Option<String>,
}

pub async fn handler(
    State(AppState { todo_use_cases, .. }): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePayload>,
) -> ApiResult<ApiTodo> {
    tracing::info!("Patch /todo/{id} | {payload:?}");

    let input = UpdateTodoInput {
        title: payload.title,
        description: payload.description,
    };

    Ok(todo_use_cases.update.execute(id, input).await?.into())
}

use axum::extract::{Path, State};

use crate::{adapters::api::error::ApiResult, app_state::AppState};

use super::ApiTodo;

pub async fn handler(
    State(AppState { todo_use_cases, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResult<ApiTodo> {
    tracing::info!("Get /todo/{id}");

    Ok(todo_use_cases.get.execute(id).await?.into())
}

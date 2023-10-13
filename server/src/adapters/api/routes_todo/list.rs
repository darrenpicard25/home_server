use axum::{extract::State, Json};

use crate::{adapters::api::error::ApiResult, app_state::AppState};

use super::ApiTodo;

pub async fn handler(
    State(AppState { todo_use_cases, .. }): State<AppState>,
) -> ApiResult<Json<Vec<ApiTodo>>> {
    tracing::info!("Get /todo");

    Ok(Json(
        todo_use_cases
            .list
            .execute()
            .await?
            .into_iter()
            .map(|entity| entity.into())
            .collect(),
    ))
}

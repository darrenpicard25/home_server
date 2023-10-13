use axum::{
    response::IntoResponse,
    routing::{patch, post},
    Json, Router,
};
use serde::Serialize;

use crate::{app_state::AppState, domain::entities::todo::Todo};

mod create;
mod get;
mod list;
mod update;

#[derive(Serialize)]
struct ApiTodo {
    id: i64,
    title: String,
    description: String,
}

impl From<Todo> for ApiTodo {
    fn from(value: Todo) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
        }
    }
}

impl IntoResponse for ApiTodo {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/todo", post(create::handler).get(list::handler))
        .route("/todo/:id", patch(update::handler).get(get::handler))
}

use axum::Router;

use crate::{app_state::AppState, error::ServiceStartupError};

pub mod error;
mod routes_hello;
mod routes_todo;

pub fn build_api_route() -> Result<Router<AppState>, ServiceStartupError> {
    Ok(Router::new().nest(
        "/api",
        Router::new()
            .merge(routes_hello::routes())
            .merge(routes_todo::routes()),
    ))
}

use app::App;
use std::net::SocketAddr;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use axum::{
    body::{boxed, Body, BoxBody},
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
    Router,
};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

use crate::{adapters::api, app_state::AppState, config::AppConfig, error::ServiceStartupError};

pub struct App {
    address: SocketAddr,
    app_config: AppConfig,
}

/// Constructor
impl App {
    pub fn new(address: SocketAddr, app_config: AppConfig) -> Self {
        Self {
            app_config,
            address,
        }
    }
}

/// Methods
impl App {
    pub async fn run(&self) -> Result<(), ServiceStartupError> {
        tracing::info!("Starting Server on: {}", self.address);

        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;

        let app_state =
            AppState::new(&self.app_config.connection_string, leptos_options.clone()).await?;

        let leptos_routes = generate_route_list(App);

        let leptos_routes = Router::new()
            .leptos_routes(&app_state, leptos_routes, || view! {  <App/> })
            .merge(api::build_api_route()?)
            .fallback(file_and_error_handler)
            .with_state(app_state);

        axum::Server::bind(&self.address)
            .serve(leptos_routes.into_make_service())
            .await
            .map_err(|_| ServiceStartupError::ServiceStartup { addr: self.address })?;

        Ok(())
    }
}

// TODO move
pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let handler =
            leptos_axum::render_app_to_stream(options.to_owned(), move || view! { <App/> });
        handler(req).await.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}

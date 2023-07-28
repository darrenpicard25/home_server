use std::net::{Ipv4Addr, SocketAddr};

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Router,
};
use clap::Parser;
use serde::Deserialize;

mod app_config;
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g. `/hello?name=Darren`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - hello handler: {:?}", "HANDLER", params);
    let name = params.name.as_deref().unwrap_or("World");

    format!("Hello, {name}!. From User-Service")
}

// e.g. `/hello/Darren`
async fn handler_hello_2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - hello handler 2: {:?}", "HANDLER", name);

    format!("Hello, {name}! From handler 2 from User-Service")
}

async fn root_handler() -> impl IntoResponse {
    println!("->> {:12} - root handler", "HANDLER");

    "User Service"
}

#[tokio::main]
async fn main() {
    const ADDRESS: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
    let app_config = app_config::AppConfig::parse();

    println!("Starting Service on {:?}:{}", ADDRESS, app_config.port);
    let router = Router::new()
        .route("/", get(root_handler))
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_hello_2));

    let address = SocketAddr::from((ADDRESS, app_config.port));

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

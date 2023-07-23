use std::net::{Ipv4Addr, SocketAddr};

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// Hello

// e.g. `/hello?name=Darren`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - hello handler: {:?}", "HANDLER", params);
    let name = params.name.as_deref().unwrap_or("World");

    format!("Hello, {name}! From Beer Service")
}

// e.g. `/hello/Darren`
async fn handler_hello_2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - hello handler 2: {:?}", "HANDLER", name);

    format!("Hello, {name}! From handler 2 in Beer Service")
}

#[tokio::main]
async fn main() {
    const PORT: u16 = 3001;
    const ADDRESS: Ipv4Addr = Ipv4Addr::UNSPECIFIED;

    println!("Starting Service on {:?}:{}", ADDRESS, PORT);
    let router = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_hello_2));

    let address = SocketAddr::from((ADDRESS, PORT));

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

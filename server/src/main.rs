use std::net::{Ipv4Addr, SocketAddr};

use app::App;
use clap::Parser;

use config::AppConfig;
use error::ServiceStartupError;

mod adapters;
mod app;
mod app_state;
mod application;
mod config;
mod domain;
mod error;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), ServiceStartupError> {
    tracing_subscriber::fmt().init();

    let app_config = AppConfig::parse();
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 8000));

    App::new(addr, app_config).run().await?;

    Ok(())
}

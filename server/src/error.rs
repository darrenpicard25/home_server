use std::net::SocketAddr;

#[derive(Debug)]
pub enum ServiceStartupError {
    ServiceStartup { addr: SocketAddr },
    DatabaseConnection(String),
    DatabaseMigration,
}

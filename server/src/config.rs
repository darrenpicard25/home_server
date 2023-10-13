use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppConfig {
    #[arg(
        long,
        default_value = "postgres://home_service_user:home_service_user@localhost:5432/home_server"
    )]
    pub connection_string: String,
}

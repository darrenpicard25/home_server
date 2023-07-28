use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct AppConfig {
    #[clap(hide(true), default_value = "beer_service")]
    pub service_name: String,

    #[clap(long, short, default_value_t = 3001)]
    pub port: u16,

    #[clap(default_value = "beer_service_db")]
    pub database: String,
    //     #[clap(long)]
    //     pub database_password: String,

    //     #[clap(long, short('u'))]
    //     pub database_user: String,
}

use sqlx::PgPool;
use std::net::TcpListener;
use env_logger::Env;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    // logger configuration
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let configuration = get_configuration().expect("Failed to read configuration");

    // tpc configuration
    let address = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    // database configuration
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to database");

    run(listener, connection_pool)?.await
}

use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    let configuration = get_configuration().unwrap();
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to database");

    run(listener, connection_pool)?.await
}

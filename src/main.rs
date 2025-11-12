use dotenvy::dotenv;
use sqlx::PgPool;
use std::net::TcpListener;
use zero::configuration::get_configuration;
use zero::startup::run;
use zero::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Set up tracing
    let subscriber = get_subscriber("zero".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Build configuration
    dotenv().ok();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to the database.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}

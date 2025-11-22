use dotenvy::dotenv;
use sqlx::PgPool;
use zero::configuration::get_configuration;
use zero::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    let subscriber = get_subscriber("zero-migrate".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Load environment variables from .env file (if present)
    dotenv().ok();

    // Build configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to the database");

    // Ensure public schema exists and user has permissions
    // Note: In DigitalOcean managed Postgres, this should already be set up
    tracing::info!("Ensuring public schema is accessible...");
    sqlx::query("CREATE SCHEMA IF NOT EXISTS public")
        .execute(&connection_pool)
        .await
        .ok(); // Ignore errors (schema likely already exists)

    // Try to grant permissions (may fail if user doesn't have GRANT privilege, which is OK)
    let username = &configuration.database.username;
    let grant_queries = vec![
        format!("GRANT USAGE ON SCHEMA public TO {}", username),
        format!("GRANT CREATE ON SCHEMA public TO {}", username),
    ];

    for query in grant_queries {
        if let Err(e) = sqlx::query(&query).execute(&connection_pool).await {
            tracing::warn!(
                "Could not grant permission (user may already have it or lack GRANT privilege): {}",
                e
            );
        }
    }

    // Run migrations
    tracing::info!("Running database migrations...");
    // Use runtime Migrator API to load migrations from the filesystem
    // This allows us to use absolute paths that work in the Docker container
    let migrations_path = std::path::Path::new("/app/migrations");
    tracing::info!("Loading migrations from: {:?}", migrations_path);

    let migrator = sqlx::migrate::Migrator::new(migrations_path)
        .await
        .expect("Failed to create migrator");

    migrator
        .run(&connection_pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Migrations completed successfully!");
    Ok(())
}

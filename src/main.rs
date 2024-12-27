use chrono::Utc;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::{debug, error, info, trace, warn, LevelFilter};
use std::time::Duration;

mod models;
mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn run_migrations(connection: &mut PgConnection) {
    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(ran_migrations) => {
            if !ran_migrations.is_empty() {
                info!(
                    "Successfully ran {} database migrations",
                    ran_migrations.len()
                );
            } else {
                info!("No migrations had to be run since the database is up to date");
            }
        }
        Err(error) => {
            error!(
                "Failed to run the database migrations. The error was: {}",
                error
            )
        }
    }
}

async fn setup_logging(logging_level: LevelFilter) {
    let mut base_config = fern::Dispatch::new();

    base_config = base_config.level(logging_level);

    let logging_target = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(std::io::stderr());

    base_config.chain(logging_target).apply().unwrap();
}
#[rocket::main]
async fn main() {
    dotenv::dotenv().ok();

    setup_logging(LevelFilter::Trace).await;

    let database_connection_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_connection_pool_manager = diesel::r2d2::ConnectionManager::new(&database_connection_url);
    let db_connection_pool = diesel::r2d2::Pool::builder()
        .max_size(15)
        .connection_timeout(Duration::from_secs(5))
        .build(db_connection_pool_manager)
        .unwrap();
    debug!("Successfully connected to the database server");

    let mut db_connection = db_connection_pool.get().unwrap_or_else(|e| {
        error!(
            "Could not get a database connection from the connection pool. The error was: {}",
            e
        );
        std::process::exit(-1);
    });
    run_migrations(&mut db_connection);
    info!("Database preparations finished");
}

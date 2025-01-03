use chrono::Utc;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::{debug, error, info, LevelFilter};
use rocket::config::{Shutdown, Sig};
use rocket::figment::{
    util::map,
    value::{Map, Value},
};
use rocket::{catchers, routes, Config as RocketConfig};
use std::path::Path;
use std::time::Duration;
use thereiwas::fairings::ThereIWasDatabaseConnection;
use thereiwas::routes::owntracks::add_new_location_record;
use thereiwas::{
    custom_handler_bad_request, custom_handler_conflict, custom_handler_internal_server_error,
    custom_handler_unprocessable_entity,
};

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

async fn setup_logging(logging_level: LevelFilter, logfile_path: &String) {
    let mut base_config = fern::Dispatch::new();
    let parsed_logfile_path = Path::new(logfile_path);

    base_config = base_config.level(logging_level);

    match std::fs::create_dir_all(parsed_logfile_path) {
        Ok(()) => { /* nothing to do here */ }
        Err(error) => {
            panic!(
                "Failed to create the output folder for the log file. The system error was: {}",
                error
            );
        }
    }

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
        .chain(std::io::stderr())
        .chain(fern::log_file(parsed_logfile_path.join("server.log")).unwrap());

    base_config
        .chain(logging_target)
        .level_for("rocket", LevelFilter::Error)
        .apply()
        .unwrap();
}

async fn get_logging_level() -> LevelFilter {
    match std::env::var("THEREIWAS_LOGGING_LEVEL")
        .unwrap_or_else(|_| "info".to_string())
        .to_lowercase()
        .as_str()
    {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => panic!("Unknown logging level"),
    }
}

#[rocket::main]
async fn main() {
    dotenv::dotenv().ok();

    let logfile_path = std::env::var("THEREIWAS_LOGFILE_PATH")
        .unwrap_or_else(|_| "/var/log/thereiwas".to_string());
    setup_logging(get_logging_level().await, &logfile_path).await;

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

    let thereiwas_database_config: Map<_, Value> = map! {
        "url" => database_connection_url.into(),
        "pool_size" => 25.into()
    };

    let rocket_configuration_figment = RocketConfig::figment()
        .merge(("databases", map!["thereiwas" => thereiwas_database_config]))
        .merge(("port", 3000))
        .merge(("address", std::net::Ipv4Addr::new(0, 0, 0, 0)))
        .merge((
            "shutdown",
            Shutdown {
                ctrlc: true,
                signals: {
                    let mut set = std::collections::HashSet::new();
                    set.insert(Sig::Term);
                    set
                },
                grace: 2,
                mercy: 3,
                force: true,
                __non_exhaustive: (),
            },
        ));

    info!("Database preparations done and starting up the API endpoints now...");
    let _ = rocket::custom(rocket_configuration_figment)
        .manage(ThereIWasDatabaseConnection::from(db_connection_pool))
        .mount("/", routes![add_new_location_record])
        .register(
            "/",
            catchers![
                custom_handler_bad_request,
                custom_handler_conflict,
                custom_handler_unprocessable_entity,
                custom_handler_internal_server_error
            ],
        )
        .launch()
        .await;
}

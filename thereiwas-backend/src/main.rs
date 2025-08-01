use chrono::Utc;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use jsonwebtoken::{DecodingKey, EncodingKey};
use log::{debug, error, info, LevelFilter};
use rocket::config::{Shutdown, Sig};
use rocket::figment::{
    util::map,
    value::{Map, Value},
};
use rocket::{catchers, routes, Config as RocketConfig};
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;
use thereiwas::fairings::ThereIWasDatabaseConnection;
use thereiwas::routes::owntracks::add_new_location_record;
use thereiwas::routes::{get_health_status, get_login_token, get_positions};
use thereiwas::{
    custom_handler_bad_request, custom_handler_conflict, custom_handler_forbidden,
    custom_handler_internal_server_error, custom_handler_not_found, custom_handler_unauthorized,
    custom_handler_unprocessable_entity, BackendConfiguration,
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
        .level_for("reqwest", LevelFilter::Error)
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

fn get_encoding_key(pem_file_path: &str) -> EncodingKey {
    match &mut File::open(pem_file_path) {
        Ok(file) => {
            let mut contents = String::new();
            if 0 == file.read_to_string(&mut contents).unwrap_or(0) {
                panic!("Could not read pem file");
            }
            match EncodingKey::from_ed_pem(contents.as_bytes()) {
                Ok(key) => key,
                Err(error) => {
                    panic!(
                        "Failed to parse encoding key. The system error was: {}",
                        error
                    );
                }
            }
        }
        Err(error) => {
            panic!(
                "Failed to open encoding key ({}). The system error was: {}",
                pem_file_path, error
            );
        }
    }
}

fn get_decoding_key(pem_file_path: &str) -> DecodingKey {
    match &mut File::open(pem_file_path) {
        Ok(file) => {
            let mut contents = String::new();
            if 0 == file.read_to_string(&mut contents).unwrap_or(0) {
                panic!("Could not read pem file");
            }
            match DecodingKey::from_ed_pem(contents.as_bytes()) {
                Ok(key) => key,
                Err(error) => {
                    panic!(
                        "Failed to parse decoding key. The system error was: {}",
                        error
                    );
                }
            }
        }
        Err(error) => {
            panic!(
                "Failed to open decoding key ({}). The system error was: {}",
                pem_file_path, error
            );
        }
    }
}

#[rocket::main]
async fn main() {
    dotenv::dotenv().ok();

    let logfile_path = std::env::var("THEREIWAS_LOGFILE_PATH")
        .unwrap_or_else(|_| "/var/log/thereiwas".to_string());
    setup_logging(get_logging_level().await, &logfile_path).await;

    let public_key_file_path = std::env::var("THEREIWAS_JWT_PUBLIC_KEY_FILE")
        .unwrap_or_else(|_| "/usr/local/thereiwas/public.key".to_string());
    let private_key_file_path = std::env::var("THEREIWAS_JWT_PRIVATE_KEY_FILE")
        .unwrap_or_else(|_| "/usr/local/thereiwas/private.key".to_string());

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

    let thereiwas_database_config: Map<_, Value> = map! { // TODO: there are two different ways for accessing the db right now
        "url" => database_connection_url.into(),
        "pool_size" => 25.into()
    };

    let backend_config = BackendConfiguration {
        api_host: "some_host".to_string(), // TODO: this
        encoding_key: Some(get_encoding_key(private_key_file_path.as_str())),
        decoding_key: Some(get_decoding_key(public_key_file_path.as_str())),
        token_audience: HashSet::new(), // TODO: this
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

    info!("Database preparations are done and starting up the API endpoints now...");
    let _ = rocket::custom(rocket_configuration_figment)
        .manage(ThereIWasDatabaseConnection::from(db_connection_pool))
        .manage(backend_config)
        .mount(
            "/v1",
            routes![
                get_login_token,
                get_health_status,
                add_new_location_record,
                get_positions
            ],
        )
        .register(
            "/",
            catchers![
                custom_handler_bad_request,
                custom_handler_unauthorized,
                custom_handler_forbidden,
                custom_handler_not_found,
                custom_handler_conflict,
                custom_handler_unprocessable_entity,
                custom_handler_internal_server_error
            ],
        )
        .launch()
        .await;
}

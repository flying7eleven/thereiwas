use chrono::Utc;
use log::{debug, error, info, trace, warn, LevelFilter};

mod models;
mod schema;

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
    setup_logging(LevelFilter::Trace).await;

    trace!("Some trace message");
    debug!("Some debug message");
    info!("Some info message");
    warn!("Some warn message");
    error!("Some error message");
}

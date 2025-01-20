use crate::fairings::ThereIWasDatabaseConnection;
use rocket::http::Status;
use rocket::{get, State};

pub mod guards;
pub mod owntracks;

#[get("/health")]
pub fn get_health_status(_db_connection_pool: &State<ThereIWasDatabaseConnection>) -> Status {
    Status::NoContent // TODO: do actual checks
}

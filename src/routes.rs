use crate::fairings::ThereIWasDatabaseConnection;
use crate::models::NewLocation;
use crate::schema;
use chrono::DateTime;
use diesel::RunQueryDsl;
use log::error;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{post, State};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewLocationRequest {
    pub batt: Option<i32>,
    pub lon: f64,
    pub acc: Option<i32>,
    pub p: Option<f64>,
    pub vac: Option<i32>,
    pub lat: f64,
    pub t: Option<String>,
    pub topic: Option<String>,
    pub m: i32,
    pub tst: i64,
    pub alt: Option<i32>,
    pub vel: Option<i32>,
    pub cog: Option<i32>,
    pub tid: String,
    pub _type: String,
    pub created_at: Option<i64>,
}

#[post("/", data = "<new_location>")]
pub fn add_new_location_record(
    db_connection_pool: &State<ThereIWasDatabaseConnection>,
    new_location: Json<NewLocationRequest>,
) -> Status {
    let new_record = NewLocation {
        horizontal_accuracy: new_location.acc,
        altitude: new_location.alt,
        latitude: new_location.lat,
        longitude: new_location.lon,
        report_trigger: new_location.t.clone(),
        measurement_time: DateTime::from_timestamp(new_location.tst, 0)
            .unwrap()
            .naive_utc(),
        vertical_accuracy: new_location.vac,
        barometric_pressure: new_location.p,
        topic: new_location.topic.clone().unwrap_or("unknown".to_string()),
        created_at: new_location.created_at.map_or(None, |time_stamp| {
            Some(DateTime::from_timestamp(time_stamp, 0).unwrap().naive_utc())
        }),
    };

    let mut db_connection = db_connection_pool.get().unwrap();
    let query_result = diesel::insert_into(schema::locations::table)
        .values(&new_record)
        .execute(&mut db_connection);

    if let Err(error) = query_result {
        error!("Could not store location data. The error was: {}", error);
        return Status::InternalServerError;
    }
    Status::NoContent
}

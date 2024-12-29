use crate::fairings::ThereIWasDatabaseConnection;
use crate::models::NewLocation;
use crate::routes::guards::RawBody;
use crate::schema;
use chrono::DateTime;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::RunQueryDsl;
use log::error;
use rocket::http::Status;
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

#[post("/owntracks", data = "<raw_body>")]
pub fn add_new_location_record(
    db_connection_pool: &State<ThereIWasDatabaseConnection>,
    raw_body: RawBody,
) -> Status {
    let new_location = match serde_json::from_slice::<NewLocationRequest>(&raw_body.0) {
        Ok(parsed) => parsed,
        Err(e) => {
            let body_str = String::from_utf8_lossy(&raw_body.0);
            error!(
                "Received unknown or invalid JSON received (error was {}): {}",
                e, body_str
            );
            return Status::UnprocessableEntity;
        }
    };

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

    if let Err(DatabaseError(error_kind, error_info)) = query_result {
        if let DatabaseErrorKind::UniqueViolation = error_kind {
            error!("Could not store the location request since the location point was already submitted");
            return Status::Conflict;
        }
        error!("There was an error reported by the database ({:?}) while storing a location request. The error was {}", error_kind, error_info.message());
        return Status::InternalServerError;
    }

    if let Err(error) = query_result {
        error!(
            "There was an error while trying to store a location request. The error was: {}",
            error
        );
        return Status::InternalServerError;
    }

    Status::NoContent
}

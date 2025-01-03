use crate::fairings::ThereIWasDatabaseConnection;
use crate::models::NewLocation;
use crate::routes::guards::RawBody;
use crate::schema;
use chrono::DateTime;
use diesel::r2d2::ConnectionManager;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::{PgConnection, RunQueryDsl};
use log::{error, trace};
use r2d2::PooledConnection;
use rocket::http::Status;
use rocket::{post, State};
use serde::{Deserialize, Serialize};

pub enum ReportTrigger {
    /// Ping issued randomly by background task (iOS,Android)
    Ping,
    /// Circular region enter/leave event (iOS,Android)
    CircularRegion,
    /// Circular region enter/leave event for +follow regions (iOS)
    CircularRegionWithFollowRegions,
    /// Beacon region enter/leave event (iOS)
    BeaconRegion,
    /// Response to a reportLocation cmd message (iOS,Android)
    ReportLocationResponse,
    /// Manual publish requested by the user (iOS,Android)
    UserRequest,
    /// Timer based publish in move (iOS)
    TimerBased,
    /// Updated by Settings/Privacy/Locations Services/System Services/Frequent Locations monitoring (iOS)
    FrequentLocationsMonitoring,
    /// The trigger which was used is not known to the server. Check logs for more information about the report trigger
    UnknownTrigger,
}

impl From<&str> for ReportTrigger {
    fn from(value: &str) -> Self {
        match value {
            "p" => ReportTrigger::Ping,
            "c" => ReportTrigger::CircularRegion,
            "C" => ReportTrigger::CircularRegion,
            "b" => ReportTrigger::BeaconRegion,
            "r" => ReportTrigger::ReportLocationResponse,
            "u" => ReportTrigger::UserRequest,
            "t" => ReportTrigger::TimerBased,
            "v" => ReportTrigger::FrequentLocationsMonitoring,
            "?" => ReportTrigger::UnknownTrigger,
            _ => {
                error!("Unknown ReportTrigger value: {}", value);
                ReportTrigger::UnknownTrigger
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct NewLocationRequest {
    pub lon: f64,
    pub lat: f64,
    pub m: i32,
    pub tst: i64,
    pub bs: Option<u8>,
    pub batt: Option<u8>,
    pub acc: Option<i32>,
    pub p: Option<f64>,
    pub vac: Option<i32>,
    pub t: Option<String>,
    pub topic: Option<String>,
    pub alt: Option<i32>,
    pub vel: Option<i32>,
    pub cog: Option<i32>,
    pub tid: String,
    pub _type: String,
    #[serde(rename = "BSSID")]
    pub bssid: Option<String>,
    #[serde(rename = "SSID")]
    pub ssid: Option<String>,
    pub conn: Option<String>,
    pub created_at: Option<i64>,
}

#[derive(Serialize, Deserialize)]
struct Waypoint {
    pub rad: i64,
    pub tst: i64,
    pub _type: String,
    pub rid: String,
    pub lon: f64,
    pub lat: f64,
    pub desc: String,
}

#[derive(Serialize, Deserialize)]
struct NewWaypointsRequest {
    pub _type: String,
    pub topic: String,
    pub waypoints: Vec<Waypoint>,
}

fn handle_new_location_request(
    location_request: &NewLocationRequest,
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Status {
    let new_record = NewLocation {
        horizontal_accuracy: location_request.acc,
        altitude: location_request.alt,
        latitude: location_request.lat,
        longitude: location_request.lon,
        report_trigger: location_request.t.clone().map_or("?".to_string(), |s| s),
        measurement_time: DateTime::from_timestamp(location_request.tst, 0)
            .unwrap()
            .naive_utc(),
        vertical_accuracy: location_request.vac,
        barometric_pressure: location_request.p,
        topic: location_request
            .topic
            .clone()
            .unwrap_or("unknown".to_string()),
        created_at: location_request.created_at.map_or(None, |time_stamp| {
            Some(DateTime::from_timestamp(time_stamp, 0).unwrap().naive_utc())
        }),
    };

    let query_result = diesel::insert_into(schema::locations::table)
        .values(&new_record)
        .execute(db_connection);

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

    trace!("Location request stored successfully");
    Status::NoContent
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
    trace!(
        "Received a new location request with the tid of {}",
        new_location.tid
    );

    let mut db_connection = db_connection_pool.get().unwrap();

    handle_new_location_request(&new_location, &mut db_connection)
}

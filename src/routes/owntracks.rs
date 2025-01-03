use crate::fairings::ThereIWasDatabaseConnection;
use crate::models::NewLocation;
use crate::routes::guards::RawBody;
use crate::schema;
use chrono::DateTime;
use diesel::r2d2::ConnectionManager;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::{PgConnection, RunQueryDsl};
use log::{debug, error, info, trace, warn};
use r2d2::PooledConnection;
use rocket::http::Status;
use rocket::{post, State};
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

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

#[derive(Deserialize)]
struct GenericRequest {
    #[serde(rename = "_type")]
    pub message_type: String,
}
#[derive(Deserialize)]
pub struct StatusRequestApple {
    #[serde(rename = "altimeterAuthorizationStatus")]
    pub altimeter_authorization_status: String,
    #[serde(rename = "altimeterIsRelativeAltitudeAvailable")]
    pub altimeter_is_relative_altitude_available: bool,
    #[serde(rename = "backgroundRefreshStatus")]
    pub background_refresh_status: String,
    #[serde(rename = "deviceIdentifierForVendor")]
    pub device_identifier_for_vendor: String,
    #[serde(rename = "deviceModel")]
    pub device_model: String,
    #[serde(rename = "deviceSystemName")]
    pub device_system_name: String,
    #[serde(rename = "deviceSystemVersion")]
    pub device_system_version: String,
    #[serde(rename = "deviceUserInterfaceIdiom")]
    pub device_user_interface_idiom: String,
    pub locale: String,
    #[serde(rename = "localeUsesMetricSystem")]
    pub locale_uses_metric_system: bool,
    #[serde(rename = "locationManagerAuthorizationStatus")]
    pub location_manager_authorization_status: String,
    pub version: String,
}

#[derive(Deserialize)]
pub struct StatusRequestAndroid {
    pub hib: i32,
    pub bo: i32,
    pub loc: i32,
    pub ps: i32,
    pub wifi: i32,
}

#[derive(Deserialize)]
struct StatusRequest {
    #[serde(rename = "iOS")]
    pub ios: Option<StatusRequestApple>,
    pub android: Option<StatusRequestAndroid>,
    #[serde(rename = "_id")]
    pub id: Option<String>,
}

#[derive(Deserialize)]
struct NewLocationRequest {
    pub lon: f64,
    pub lat: f64,
    // pub m: i32,
    pub tst: i64,
    // pub bs: Option<u8>,
    // pub batt: Option<u8>,
    pub acc: Option<i32>,
    pub p: Option<f64>,
    pub vac: Option<i32>,
    pub t: Option<String>,
    pub topic: Option<String>,
    pub alt: Option<i32>,
    // pub vel: Option<i32>,
    // pub cog: Option<i32>,
    pub tid: String,
    // #[serde(rename = "_type")]
    // pub message_type: String,
    // #[serde(rename = "BSSID")]
    // pub bssid: Option<String>,
    // #[serde(rename = "SSID")]
    // pub ssid: Option<String>,
    // pub conn: Option<String>,
    pub created_at: Option<i64>,
}

#[derive(Debug)]
enum OwnTracksError {
    /// Each location can only be stored once. If a second request will result in an error
    LocationAlreadyKnown,
    /// There was a generic database error while storing an entity. See the logfiles for more information
    GenericDatabaseError,
    /// The request body of the request could not be parsed. See the logfile for more information
    RequestBodyParsingError,
}

impl Display for OwnTracksError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OwnTracksError::LocationAlreadyKnown => {
                write!(f, "The provided location is already known")
            }
            OwnTracksError::GenericDatabaseError => write!(
                f,
                "There was an generic database error while trying to query or save an entity"
            ),
            OwnTracksError::RequestBodyParsingError => write!(
                f,
                "There was an error while trying to parse the request body to the expected data type"
            ),
        }
    }
}

impl Error for OwnTracksError {}

fn handle_status_request(raw_body: &RawBody) -> Result<(), OwnTracksError> {
    let status_request = match serde_json::from_slice::<StatusRequest>(&raw_body.0) {
        Ok(parsed) => parsed,
        Err(e) => {
            let body_str = String::from_utf8_lossy(&raw_body.0);
            error!(
                "Received unknown or invalid JSON received (error was {}): {}",
                e, body_str
            );
            return Err(OwnTracksError::RequestBodyParsingError);
        }
    };
    trace!("Received a new status request");

    if let Some(ios_status) = status_request.ios {
        info!("Status information for iOS:");
        info!(
            "  Altimeter authorization status: {}",
            ios_status.altimeter_authorization_status
        );
        info!(
            "  Is relative altitude available: {}",
            ios_status.altimeter_is_relative_altitude_available
        );
        info!(
            "  Background refresh status: {}",
            ios_status.background_refresh_status
        ); // TODO: better description
        info!(
            "  Device identifier: {}",
            ios_status.device_identifier_for_vendor
        );
        info!("  Device model: {}", ios_status.device_model);
        info!("  Device system name: {}", ios_status.device_system_name);
        info!(
            "  Device system version: {}",
            ios_status.device_system_version
        );
        info!(
            "  Device UI idiom: {}",
            ios_status.device_user_interface_idiom
        );
        info!("  Locale: {}", ios_status.locale);
        info!(
            "  Locale uses metric system: {}",
            ios_status.locale_uses_metric_system
        );
        info!(
            "  Location manager authorization status: {}",
            ios_status.location_manager_authorization_status
        );
        info!("  OwnTracks App version: {}", ios_status.version);
    }
    if let Some(android_status) = status_request.android {
        info!("Status information for Android:");
        if let Some(request_id) = status_request.id {
            info!("  Request id: {}", request_id);
        }
        info!("  HIB: {}", android_status.hib); // TODO: better description
        info!("  BO: {}", android_status.bo); // TODO: better description
        info!("  LOC: {}", android_status.loc); // TODO: better description
        info!("  PS: {}", android_status.ps); // TODO: better description
        info!("  WIFI: {}", android_status.wifi); // TODO: better description
    }

    Ok(())
}

fn handle_new_location_request(
    raw_body: &RawBody,
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<(), OwnTracksError> {
    let location_request = match serde_json::from_slice::<NewLocationRequest>(&raw_body.0) {
        Ok(parsed) => parsed,
        Err(e) => {
            let body_str = String::from_utf8_lossy(&raw_body.0);
            error!(
                "Received unknown or invalid JSON received (error was {}): {}",
                e, body_str
            );
            return Err(OwnTracksError::RequestBodyParsingError);
        }
    };
    trace!(
        "Received a new location request with the tid of {}",
        location_request.tid
    );

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
            return Err(OwnTracksError::LocationAlreadyKnown);
        }
        error!("There was an error reported by the database ({:?}) while storing a location request. The error was {}", error_kind, error_info.message());
        return Err(OwnTracksError::GenericDatabaseError);
    }

    if let Err(error) = query_result {
        error!(
            "There was an error while trying to store a location request. The error was: {}",
            error
        );
        return Err(OwnTracksError::GenericDatabaseError);
    }

    debug!("Location request stored successfully");
    Ok(())
}

#[post("/owntracks", data = "<raw_body>")]
pub fn add_new_location_record(
    db_connection_pool: &State<ThereIWasDatabaseConnection>,
    raw_body: RawBody,
) -> Status {
    let generic_request = match serde_json::from_slice::<GenericRequest>(&raw_body.0) {
        Ok(parsed) => parsed,
        Err(e) => {
            let body_str = String::from_utf8_lossy(&raw_body.0);
            error!(
                "The received request body can not be interpreted (error was {}): {}",
                e, body_str
            );
            return Status::UnprocessableEntity;
        }
    };
    trace!(
        "Received OwnTracks request of type '{}'",
        generic_request.message_type
    );

    let mut db_connection = db_connection_pool.get().unwrap();

    let message_handling_result = match generic_request.message_type.as_str() {
        "location" => handle_new_location_request(&raw_body, &mut db_connection),
        "status" => handle_status_request(&raw_body),
        _ => {
            warn!(
                "There is no implementation for handling {} requests yet",
                generic_request.message_type
            );
            return Status::BadRequest;
        }
    };

    match message_handling_result {
        Ok(_) => Status::NoContent,
        Err(error) => match error {
            OwnTracksError::LocationAlreadyKnown => Status::Conflict,
            OwnTracksError::RequestBodyParsingError => Status::UnprocessableEntity,
            OwnTracksError::GenericDatabaseError => Status::InternalServerError,
        },
    }
}

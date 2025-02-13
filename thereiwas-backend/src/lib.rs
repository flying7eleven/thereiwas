use crate::models::NewAuditLog;
use chrono::Utc;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use jsonwebtoken::{DecodingKey, EncodingKey};
use lazy_static::lazy_static;
use log::error;
use r2d2::PooledConnection;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{catch, Request};
use serde::Deserialize;
use std::collections::HashSet;
use std::fmt;

pub mod fairings;
mod guards;
pub mod models;
pub mod routes;
pub mod schema;

lazy_static! {
    /// The time in seconds a token is valid.
    static ref TOKEN_LIFETIME_IN_SECONDS: usize = 60 * 60;
}

#[derive(Serialize)]
pub struct CustomHandlerError {
    message: String,
}

#[catch(400)]
pub fn custom_handler_bad_request(req: &Request<'_>) -> Json<CustomHandlerError> {
    Json(CustomHandlerError {
        message: format!("Request to {} was not correct", req.uri()),
    })
}

#[catch(401)]
pub fn custom_handler_unauthorized(req: &Request<'_>) -> Json<CustomHandlerError> {
    Json(CustomHandlerError {
        message: format!("Request to {} was unauthorized", req.uri()),
    })
}

#[catch(403)]
pub fn custom_handler_forbidden(req: &Request<'_>) -> Json<CustomHandlerError> {
    Json(CustomHandlerError {
        message: format!("Request to {} was not authorized", req.uri()),
    })
}

#[catch(404)]
pub fn custom_handler_not_found(req: &Request<'_>) -> Json<CustomHandlerError> {
    Json(CustomHandlerError {
        message: format!("Could not find resource {}", req.uri()),
    })
}

#[catch(409)]
pub fn custom_handler_conflict(_: &Request<'_>) -> Json<CustomHandlerError> {
    Json(CustomHandlerError {
        message: "The submitted data point seems to be submitted and stored before already"
            .to_string(),
    })
}

#[catch(422)]
pub fn custom_handler_unprocessable_entity(_: &Request<'_>) -> Json<CustomHandlerError> {
    Json(CustomHandlerError {
        message: "The request was well-formed but was unable to be followed due to semantic errors"
            .to_string(),
    })
}

#[catch(500)]
pub fn custom_handler_internal_server_error(_: &Request<'_>) -> Json<CustomHandlerError> {
    Json(CustomHandlerError {
        message: "Internal Server Error".to_string(),
    })
}

pub enum AuditLogAction {
    ClientTokenAuthentication,
    UserAuthentication,
}

impl fmt::Display for AuditLogAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuditLogAction::ClientTokenAuthentication => write!(f, "client_token_authentication"),
            AuditLogAction::UserAuthentication => write!(f, "user_authentication"),
        }
    }
}

pub enum AuditLogResult {
    Successful,
    Failed,
}

impl fmt::Display for AuditLogResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuditLogResult::Successful => write!(f, "successful"),
            AuditLogResult::Failed => write!(f, "failed"),
        }
    }
}

pub fn log_audit_message(
    db_coonection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    auth_type: AuditLogAction,
    auth_result: AuditLogResult,
    request_source: &str,
) {
    let new_authorization_request = NewAuditLog {
        request_time: Utc::now().naive_utc(),
        action: auth_type.to_string(),
        result: auth_result.to_string(),
        source: request_source.to_owned(),
    };

    if let Err(error) = diesel::insert_into(schema::audit_log::table)
        .values(&new_authorization_request)
        .execute(db_coonection)
        .map(|query_result| {
            if query_result != 1 {
                error!(
                    "Failed to insert audit log entry into database; {} rows changed",
                    query_result
                );
            }
        })
    {
        error!(
            "Failed to insert audit log entry into database; the reported error was: {}",
            error
        );
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    iat: usize,
    nbf: usize,
    sub: String,
    iss: String,
    aud: HashSet<String>,
}

pub fn get_token_for_user(
    subject: &str,
    audience: HashSet<String>,
    issuer: String,
    encoding_key: &EncodingKey,
) -> Option<String> {
    use jsonwebtoken::{encode, Algorithm, Header};
    use log::error;
    use std::time::{SystemTime, UNIX_EPOCH};

    // get the issuing time for the token
    let token_issued_at = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as usize,
        Err(error) => {
            error!(
                "Could not get the issuing time for the token. The error was: {}",
                error
            );
            return None;
        }
    };

    // calculate the time when the token expires
    let token_expires_at = token_issued_at + 1 + *TOKEN_LIFETIME_IN_SECONDS;

    // define the content of the actual token
    let token_claims = Claims {
        exp: token_expires_at,
        iat: token_issued_at,
        nbf: token_issued_at + 1,
        sub: subject.to_owned(),
        iss: issuer,
        aud: audience,
    };

    // generate a new JWT for the supplied header and token claims. if we were successful, return
    // the token
    let header = Header::new(Algorithm::EdDSA);
    match encode(&header, &token_claims, encoding_key) {
        Ok(token) => Some(token),
        Err(error) => {
            error!("Could not encode JWT token; {}", error);
            None
        }
    }
}

#[derive(Clone)]
pub struct BackendConfiguration {
    /// The host base URL of the API (e.g. https://www.example.com; without a path like /api).
    pub api_host: String,
    /// The key which is used to encode a token signature.
    pub encoding_key: Option<EncodingKey>,
    /// The key which is used to decode a token signature.
    pub decoding_key: Option<DecodingKey>,
    /// A list of URLs which represent the audience for this token.
    pub token_audience: HashSet<String>,
}

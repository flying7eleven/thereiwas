use crate::models::NewAuditLog;
use chrono::Utc;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use log::error;
use r2d2::PooledConnection;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{catch, Request};
use std::fmt;

pub mod fairings;
mod guards;
pub mod models;
pub mod routes;
pub mod schema;

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
}

impl fmt::Display for AuditLogAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuditLogAction::ClientTokenAuthentication => write!(f, "client_token_authentication"),
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

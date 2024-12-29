use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{catch, Request};

pub mod fairings;
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

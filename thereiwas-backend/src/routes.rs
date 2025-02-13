use crate::fairings::ThereIWasDatabaseConnection;
use crate::models::User;
use crate::schema::users::dsl::users;
use crate::schema::users::username;
use crate::{
    get_token_for_user, log_audit_message, AuditLogAction, AuditLogResult, BackendConfiguration,
};
use bcrypt::verify;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post, State};
use serde::{Deserialize, Serialize};

pub mod guards;
pub mod owntracks;

#[get("/health")]
pub fn get_health_status(_db_connection_pool: &State<ThereIWasDatabaseConnection>) -> Status {
    Status::NoContent // TODO: do actual checks
}

#[derive(Deserialize)]
pub struct LoginInformation {
    /// The username of the user.
    username: String,
    /// The password for the login request.
    password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    /// The access token to use for API requests.
    access_token: String,
}

#[post("/auth/token", data = "<login_information>")]
pub fn get_login_token(
    db_connection_pool: &State<ThereIWasDatabaseConnection>,
    login_information: Json<LoginInformation>,
    config: &State<BackendConfiguration>,
) -> Result<Json<TokenResponse>, Status> {
    let mut db_connection = db_connection_pool.get().unwrap();

    // try to get the user record for the supplied username
    let supplied_username = login_information.username.clone();
    let maybe_user_result = db_connection
        .build_transaction()
        .read_only()
        .run::<_, diesel::result::Error, _>(move |connection| {
            if let Ok(found_users) = users
                .filter(username.eq(supplied_username))
                .load::<User>(connection)
            {
                // if we did not get exactly one user, return an 'error'
                if found_users.len() != 1 {
                    return Err(diesel::result::Error::NotFound);
                }

                // return the found user
                return Ok(found_users[0].clone());
            }

            //
            Err(diesel::result::Error::NotFound) // TODO: not the real error
        });

    // try to get the actual user object or delay a bit and then return with the corresponding error
    let user = match maybe_user_result {
        Ok(user) => user,
        Err(_) => {
            // ensure that we know what happened
            error!(
                "Could not get the user record for '{}'",
                login_information.username
            );

            // just slow down the process to prevent easy checking if a username exists or not
            let _ = verify(
                "some_password",
                "$2y$12$7xMzqvnHyizkumZYpIRXheGMAqDKVo8HKtpmQSn51JUfY0N2VN4ua",
            );

            // log the failed attempt
            log_audit_message(
                &mut db_connection,
                AuditLogAction::UserAuthentication,
                AuditLogResult::Failed,
                "<unknown>",
            ); // TODO: ip address

            // finally we can tell teh user that he/she is not authorized
            return Err(Status::Unauthorized);
        }
    };

    // check if the supplied password matches the one we stored in the database using the same bcrypt
    // parameters
    match verify(&login_information.password, user.password_hash.as_str()) {
        Ok(is_password_correct) => {
            if !is_password_correct {
                log_audit_message(
                    &mut db_connection,
                    AuditLogAction::UserAuthentication,
                    AuditLogResult::Failed,
                    "<unknown>",
                ); // TODO: ip address

                return Err(Status::Unauthorized);
            }
        }
        Err(error) => {
            error!("Could not verify the supplied password with the one stored in the database. The error was: {}", error);
            return Err(Status::InternalServerError);
        }
    }

    // if we get here, we ensured that the user is known and that the supplied password
    // was valid, we can generate a new access token and return it to the calling party
    if let Some(token) = get_token_for_user(
        &login_information.username,
        config.token_audience.clone(),
        "".to_string(), // TODO: set the correct token issuer
        &config.encoding_key.clone().unwrap(),
    ) {
        log_audit_message(
            &mut db_connection,
            AuditLogAction::UserAuthentication,
            AuditLogResult::Successful,
            "<unknown>",
        ); // TODO: ip address

        return Ok(Json(TokenResponse {
            access_token: token,
        }));
    }

    // it seems that we failed to generate a valid token, this should never happen, something
    // seems to be REALLY wrong
    Err(Status::InternalServerError)
}

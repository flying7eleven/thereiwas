use crate::fairings::ThereIWasDatabaseConnection;
use crate::models::{ClientToken, NewAuthorizationRequest};
use crate::schema;
use crate::schema::client_tokens::dsl::client_tokens;
use crate::schema::client_tokens::{client as client_id_column, secret as client_secret_column};
use chrono::Utc;
use diesel::r2d2::ConnectionManager;
use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use log::{debug, error, warn};
use r2d2::PooledConnection;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};
use std::fmt;
use std::net::{IpAddr, SocketAddr};

pub struct AuthenticatedClient {
    pub id: i32,
}

#[derive(Debug)]
pub enum AuthorizationError {
    /// Could not find any authentication URL parameters in the request
    MissingAuthorizationUrlParameter,
    /// Could not find the database connection pool we need
    DatabaseConnectionPoolNotFound,
    /// There was a generic database error which prevented to fetch information
    DatabaseError,
}

enum AuthorizationType {
    ClientToken,
}

impl fmt::Display for AuthorizationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthorizationType::ClientToken => write!(f, "ClientToken"),
        }
    }
}

enum AuthorizationResult {
    Successful,
    Failed,
}

impl fmt::Display for AuthorizationResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthorizationResult::Successful => write!(f, "Successful"),
            AuthorizationResult::Failed => write!(f, "Failed"),
        }
    }
}

fn log_authentication_attempt(
    db_coonection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    auth_type: AuthorizationType,
    auth_result: AuthorizationResult,
    identification_principle: Option<&String>,
    request_source: &String,
) {
    let new_authorization_request = NewAuthorizationRequest {
        request_time: Utc::now().naive_utc(),
        auth_type: auth_type.to_string(),
        auth_result: auth_result.to_string(),
        identification_principle: identification_principle.and_then(|x| Some(x.clone())),
        source: request_source.clone(),
    };

    if let Err(error) = diesel::insert_into(schema::authorization_requests::table)
        .values(&new_authorization_request)
        .execute(db_coonection)
        .map(|query_result| {
            if query_result != 1 {
                error!(
                    "Failed to insert authorization request for principal {} into database",
                    identification_principle.unwrap_or(&"<unknown>".to_string())
                );
            }
        })
    {
        error!(
            "Failed to insert authorization request for principal {}. The error was: {}",
            identification_principle.unwrap(),
            error
        );
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedClient {
    type Error = AuthorizationError;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<AuthenticatedClient, AuthorizationError> {
        let remote_endppoint = request
            .real_ip()
            .unwrap_or(
                request
                    .remote()
                    .unwrap_or(SocketAddr::new(IpAddr::from([0, 0, 0, 0]), 0))
                    .ip(),
            )
            .to_string();
        if let Some(maybe_client_id) = request.query_value::<String>("client") {
            if let Some(maybe_client_secret) = request.query_value::<String>("secret") {
                let client_id = maybe_client_id.unwrap(); // TODO: better handling
                let client_secret = maybe_client_secret.unwrap(); // TODO: better handling

                let db_connection_pool_state = match request
                    .guard::<&State<ThereIWasDatabaseConnection>>()
                    .await
                {
                    Outcome::Success(state) => state,
                    Outcome::Error(_) | Outcome::Forward(_) => {
                        error!("Failed to get database connection pool from the application managed state");
                        return Outcome::Error((
                            Status::InternalServerError,
                            AuthorizationError::DatabaseConnectionPoolNotFound,
                        ));
                    }
                };
                let mut db_connection_pool = db_connection_pool_state.get().unwrap();

                match client_tokens
                    .filter(
                        client_id_column
                            .eq(client_id.to_uppercase())
                            .and(client_secret_column.eq(client_secret)),
                    )
                    .load::<ClientToken>(&mut db_connection_pool)
                {
                    Ok(matching_client_tokens) => {
                        if let Some(client_token) = matching_client_tokens.get(0) {
                            debug!("Successfully found valid token. Authenticating client with the id {}", client_token.id);
                            log_authentication_attempt(
                                &mut db_connection_pool,
                                AuthorizationType::ClientToken,
                                AuthorizationResult::Successful,
                                Some(&client_id),
                                &format!("Request originated from {}", remote_endppoint),
                            );
                            return Outcome::Success(AuthenticatedClient {
                                id: client_token.id,
                            });
                        }
                        warn!("Could not find a matching client_id and client_secret pair in the database");
                        log_authentication_attempt(
                            &mut db_connection_pool,
                            AuthorizationType::ClientToken,
                            AuthorizationResult::Failed,
                            Some(&client_id),
                            &format!("Request originated from {}", remote_endppoint),
                        );
                        return Outcome::Error((
                            Status::Forbidden,
                            AuthorizationError::MissingAuthorizationUrlParameter,
                        ));
                    }
                    Err(e) => {
                        error!("Failed to query the client token for the client with the id of {}. The error was: {}", client_id, e);
                        log_authentication_attempt(
                            &mut db_connection_pool,
                            AuthorizationType::ClientToken,
                            AuthorizationResult::Failed,
                            Some(&client_id),
                            &format!("Request originated from {}", remote_endppoint),
                        );
                        return Outcome::Error((
                            Status::InternalServerError,
                            AuthorizationError::DatabaseError,
                        ));
                    }
                }
            }
        }

        warn!("Could not find the client_id or the client_secret in the URL parameters of the request");
        Outcome::Error((
            Status::Forbidden,
            AuthorizationError::MissingAuthorizationUrlParameter,
        ))
    }
}

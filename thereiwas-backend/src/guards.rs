use crate::fairings::ThereIWasDatabaseConnection;
use crate::models::ClientToken;
use crate::schema::client_tokens::dsl::client_tokens;
use crate::schema::client_tokens::{client as client_id_column, secret as client_secret_column};
use crate::{log_audit_message, AuditLogAction, AuditLogResult};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use log::{debug, error, warn};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};
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
        if let Some(maybe_client_id) = request.query_value::<String>("client_id") {
            if let Some(maybe_client_secret) = request.query_value::<String>("client_secret") {
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
                            .eq(&client_id)
                            .and(client_secret_column.eq(client_secret)),
                    )
                    .load::<ClientToken>(&mut db_connection_pool)
                {
                    Ok(matching_client_tokens) => {
                        if let Some(client_token) = matching_client_tokens.first() {
                            debug!("Successfully found valid token. Authenticating client with the id {}", client_token.id);
                            log_audit_message(
                                &mut db_connection_pool,
                                AuditLogAction::ClientTokenAuthentication,
                                AuditLogResult::Successful,
                                &format!("Request originated from {}", remote_endppoint),
                            );
                            return Outcome::Success(AuthenticatedClient {
                                id: client_token.id,
                            });
                        }
                        warn!("Could not find a matching client_id and client_secret pair in the database");
                        log_audit_message(
                            &mut db_connection_pool,
                            AuditLogAction::ClientTokenAuthentication,
                            AuditLogResult::Failed,
                            &format!("Request originated from {}", remote_endppoint),
                        );
                        return Outcome::Error((
                            Status::Forbidden,
                            AuthorizationError::MissingAuthorizationUrlParameter,
                        ));
                    }
                    Err(e) => {
                        error!("Failed to query the client token for the client with the id of {}. The error was: {}", client_id, e);
                        log_audit_message(
                            &mut db_connection_pool,
                            AuditLogAction::ClientTokenAuthentication,
                            AuditLogResult::Failed,
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

use log::error;
use rocket::data::{FromData, Outcome, ToByteUnit};
use rocket::http::Status;
use rocket::tokio::io::AsyncReadExt;
use rocket::{Data, Request};

pub struct RawBody(pub Vec<u8>);

#[rocket::async_trait]
impl<'r> FromData<'r> for RawBody {
    type Error = std::io::Error;

    async fn from_data(_: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let limit = 1.mebibytes();

        let mut stream = data.open(limit);
        let mut buffer = Vec::new();

        match stream.read_to_end(&mut buffer).await {
            Ok(_) => Outcome::Success(RawBody(buffer)),
            Err(e) => {
                error!("Failed to read body content from request: {}", e);
                Outcome::Error((Status::InternalServerError, e))
            }
        }
    }
}

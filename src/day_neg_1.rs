use rocket::{get, http::Status};

#[get("/-1/error")]
pub fn error() -> Status {
    Status::InternalServerError
}

use rocket::{get, http::Status};

#[get("/error")]
pub fn error() -> Status {
    Status::InternalServerError
}

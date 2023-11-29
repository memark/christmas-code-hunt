use rocket::{get, http::Status, launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, error])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/-1/error")]
fn error() -> Status {
    Status::InternalServerError
}

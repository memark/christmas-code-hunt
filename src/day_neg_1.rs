use rocket::{get, http::Status, routes, Route};

pub fn get_routes() -> Vec<Route> {
    routes![error]
}

#[get("/error")]
fn error() -> Status {
    Status::InternalServerError
}

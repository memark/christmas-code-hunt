use rocket::{get, http::Status, routes};
use std::{ops::BitXor, path::PathBuf};

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, error, day_1]);

    Ok(rocket.into())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/-1/error")]
fn error() -> Status {
    Status::InternalServerError
}

#[get("/1/<nums..>")]
fn day_1(nums: PathBuf) -> String {
    nums.iter()
        .map(|x| x.to_str().unwrap().parse().unwrap())
        .reduce(i32::bitxor)
        .unwrap()
        .pow(3)
        .to_string()
}

use rocket::{get, routes, Route};
use std::{ops::BitXor, path::PathBuf};

pub fn get_routes() -> Vec<Route> {
    routes![nums]
}

#[get("/<nums..>")]
fn nums(nums: PathBuf) -> String {
    nums.iter()
        .map(|x| x.to_str().unwrap().parse().unwrap())
        .reduce(i32::bitxor)
        .unwrap()
        .pow(3)
        .to_string()
}

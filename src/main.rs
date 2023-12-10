use fancy_regex::Regex;
use rocket::serde::json::Json;
use rocket::{get, http::Status, post, routes};
use serde::Serialize;
use std::{ops::BitXor, path::PathBuf};

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, error, day_1, day_6]);

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

#[post("/6", data = "<input>")]
fn day_6(input: &str) -> Json<ElfCount> {
    let elf = input.match_indices("elf").count();
    let elf_on_a_shelf = Regex::new("(?<=elf on a )shelf")
        .unwrap()
        .find_iter(input)
        .count();
    let shelf_with_no_elf_on_it = Regex::new("(?<!elf on a )shelf")
        .unwrap()
        .find_iter(input)
        .count();

    Json(ElfCount {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf_on_it,
    })
}

#[derive(Serialize)]
struct ElfCount {
    elf: usize,

    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,

    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

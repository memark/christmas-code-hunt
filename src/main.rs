use fancy_regex::Regex;
use rocket::serde::json::Json;
use rocket::{get, http::Status, post, routes};
use serde::{Deserialize, Serialize};
use std::{ops::BitXor, path::PathBuf};

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount(
        "/",
        routes![index, error, day_1, day_4_strength, day_4_contest, day_6],
    );

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

#[post("/4/strength", data = "<input>")]
fn day_4_strength(input: Json<Vec<StrengthReindeer>>) -> String {
    input.iter().map(|x| x.strength).sum::<u32>().to_string()
}

#[derive(Deserialize)]
struct StrengthReindeer {
    strength: u32,
}

#[post("/4/contest", data = "<input>")]
fn day_4_contest(input: Json<Vec<ContestReindeer>>) -> Json<ReindeerContestResult> {
    let fastest = input
        .iter()
        .max_by(|x, y| f32::total_cmp(&x.speed, &y.speed))
        .unwrap();
    let tallest = input.iter().max_by_key(|x| x.height).unwrap();
    let magician = input.iter().max_by_key(|x| x.snow_magic_power).unwrap();
    let consumer = input
        .iter()
        .max_by_key(|x| x.candies_eaten_yesterday)
        .unwrap();

    Json(ReindeerContestResult {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })
}

#[derive(Deserialize)]
struct ContestReindeer {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

#[derive(Serialize)]
struct ReindeerContestResult {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
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

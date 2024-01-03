use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};

#[post("/4/strength", data = "<input>")]
pub fn strength(input: Json<Vec<StrengthReindeer>>) -> String {
    input.iter().map(|x| x.strength).sum::<u32>().to_string()
}

#[derive(Deserialize)]
pub struct StrengthReindeer {
    strength: u32,
}

#[post("/4/contest", data = "<input>")]
pub fn contest(input: Json<Vec<ContestReindeer>>) -> Json<ReindeerContestResult> {
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
pub struct ContestReindeer {
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
pub struct ReindeerContestResult {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

use base64::Engine;
use fancy_regex::Regex;
use rocket::{
    get,
    http::{CookieJar, Status},
    post, routes,
    serde::json::Json,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::BitXor, path::PathBuf};

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount(
        "/",
        routes![
            index,
            error,
            day_1,
            day_4_strength,
            day_4_contest,
            day_6,
            day_7_decode,
            day_7_bake,
            day_7_bake2
        ],
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

#[get("/7/decode")]
fn day_7_decode(cookies: &CookieJar<'_>) -> String {
    let base64 = cookies.get("recipe").unwrap().value();
    let bytes = base64::prelude::BASE64_STANDARD.decode(base64).unwrap();
    String::from_utf8(bytes).unwrap()
}

#[get("/7/bake")]
fn day_7_bake(cookies: &CookieJar<'_>) -> String {
    let base64 = cookies.get("recipe").unwrap().value();
    let bytes = base64::prelude::BASE64_STANDARD.decode(base64).unwrap();
    let string = String::from_utf8(bytes).unwrap();
    let request = serde_json::from_str::<BakeRequest>(&string).unwrap();
    println!("{request:#?}");

    let flour = request.pantry.flour / request.recipe.flour;
    let sugar = request.pantry.sugar / request.recipe.sugar;
    let butter = request.pantry.butter / request.recipe.butter;
    let baking_powder = request.pantry.baking_powder / request.recipe.baking_powder;
    let chocolate_chips = request.pantry.chocolate_chips / request.recipe.chocolate_chips;

    let num_cookies = [flour, sugar, butter, baking_powder, chocolate_chips]
        .into_iter()
        .min()
        .unwrap();

    let response = BakeResponse {
        cookies: num_cookies,
        pantry: Ingredients {
            flour: request.pantry.flour - request.recipe.flour * num_cookies,
            sugar: request.pantry.sugar - request.recipe.sugar * num_cookies,
            butter: request.pantry.butter - request.recipe.butter * num_cookies,
            baking_powder: request.pantry.baking_powder
                - request.recipe.baking_powder * num_cookies,
            chocolate_chips: request.pantry.chocolate_chips
                - request.recipe.chocolate_chips * num_cookies,
        },
    };
    println!("{response:#?}");

    serde_json::to_string_pretty(&response).unwrap()
}

#[derive(Debug, Deserialize)]
struct BakeRequest {
    recipe: Ingredients,
    pantry: Ingredients,
}

#[derive(Debug, Deserialize, Serialize)]
struct Ingredients {
    flour: u64,
    sugar: u64,
    butter: u64,
    #[serde(rename = "baking powder")]
    baking_powder: u64,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: u64,
}

#[derive(Debug, Serialize)]
struct BakeResponse {
    cookies: u64,
    pantry: Ingredients,
}

#[get("/7/bake2")]
fn day_7_bake2(cookies: &CookieJar<'_>) -> String {
    let base64 = cookies.get("recipe").unwrap().value();
    let bytes = base64::prelude::BASE64_STANDARD.decode(base64).unwrap();
    let string = String::from_utf8(bytes).unwrap();
    let request = serde_json::from_str::<BakeRequest2>(&string).unwrap();
    println!("{request:#?}");

    // let flour = request.pantry.flour / request.recipe.flour;

    // let num_cookies = [flour, sugar, butter, baking_powder, chocolate_chips]
    //     .into_iter()
    //     .min()
    //     .unwrap();

    let response = BakeResponse2 {
        cookies: 0,
        pantry: HashMap::default(),
    };
    println!("{response:#?}");

    serde_json::to_string_pretty(&response).unwrap()

    // num_cookies.to_string()
}

#[derive(Debug, Deserialize)]
struct BakeRequest2 {
    recipe: HashMap<String, u64>,
    pantry: HashMap<String, u64>,
}

#[derive(Debug, Serialize)]
struct BakeResponse2 {
    cookies: u64,
    pantry: HashMap<String, u64>,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    username: String,

    #[serde(flatten)]
    extra: HashMap<String, u64>,
}

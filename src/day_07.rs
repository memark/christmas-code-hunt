use base64::Engine;
use rocket::{get, http::CookieJar, routes, Route};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn get_routes() -> Vec<Route> {
    routes![decode, bake, bake2]
}

#[get("/decode")]
fn decode(cookies: &CookieJar<'_>) -> String {
    let base64 = cookies.get("recipe").unwrap().value();
    let bytes = base64::prelude::BASE64_STANDARD.decode(base64).unwrap();
    String::from_utf8(bytes).unwrap()
}

#[get("/bake")]
fn bake(cookies: &CookieJar<'_>) -> String {
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

#[get("/bake2")]
fn bake2(cookies: &CookieJar<'_>) -> String {
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
    #[allow(dead_code)]
    recipe: HashMap<String, u64>,
    #[allow(dead_code)]
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

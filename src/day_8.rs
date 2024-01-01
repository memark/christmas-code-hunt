use rocket::get;
use serde::Deserialize;

#[get("/8/weight/<pokedex_number>")]
pub async fn weight(pokedex_number: u32) -> String {
    let weight_kg = get_weight_in_kg_for_pokemon(pokedex_number).await;

    weight_kg.to_string()
}

#[get("/8/drop/<pokedex_number>")]
pub async fn drop(pokedex_number: u32) -> String {
    let weight_kg = get_weight_in_kg_for_pokemon(pokedex_number).await;
    let momentum = weight_kg * get_vel_from_drop_height(10.0);

    momentum.to_string()
}

async fn get_weight_in_kg_for_pokemon(id: u32) -> f32 {
    reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{id}"))
        .await
        .unwrap()
        .json::<Pokemon>()
        .await
        .unwrap()
        .weight as f32
        / 10.0
}

#[derive(Deserialize)]
struct Pokemon {
    weight: u32,
}

fn get_vel_from_drop_height(height: f32) -> f32 {
    (2.0 * 9.825 * height).sqrt()
}

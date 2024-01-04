use fancy_regex::Regex;
use rocket::{post, routes, serde::json::Json, Route};
use serde::Serialize;

pub fn get_routes() -> Vec<Route> {
    routes![elf_count]
}

#[post("/", data = "<input>")]
fn elf_count(input: &str) -> Json<ElfCount> {
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

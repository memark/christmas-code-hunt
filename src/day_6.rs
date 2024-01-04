use fancy_regex::Regex;
use rocket::{post, serde::json::Json};
use serde::Serialize;

#[post("/", data = "<input>")]
pub fn elf_count(input: &str) -> Json<ElfCount> {
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
pub struct ElfCount {
    elf: usize,

    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,

    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

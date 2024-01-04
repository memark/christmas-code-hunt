mod day_1;
mod day_4;
mod day_6;
mod day_7;
mod day_8;
mod day_neg_1;
mod root;

use rocket::routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![root::index])
        .mount("/-1", routes![day_neg_1::error])
        .mount("/1", routes![day_1::nums])
        .mount("/4", routes![day_4::strength, day_4::contest])
        .mount("/6", routes![day_6::elf_count])
        .mount("/7", routes![day_7::decode, day_7::bake, day_7::bake2,])
        .mount("/8", routes![day_8::weight, day_8::drop]);

    Ok(rocket.into())
}

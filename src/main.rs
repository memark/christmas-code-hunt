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
    let rocket = rocket::build().mount(
        "/",
        routes![
            root::index,
            day_neg_1::error,
            day_1::nums,
            day_4::strength,
            day_4::contest,
            day_6::elf_count,
            day_7::decode,
            day_7::bake,
            day_7::bake2,
            day_8::weight,
            day_8::drop
        ],
    );

    Ok(rocket.into())
}

use shuttle_rocket::ShuttleRocket;
use shuttle_runtime::main;

mod day_01;
mod day_04;
mod day_06;
mod day_07;
mod day_08;
mod day_11;
mod day_neg_01;
mod root;

#[main]
async fn main() -> ShuttleRocket {
    let rocket = rocket::build()
        .mount("/-1", day_neg_01::get_routes())
        .mount("/", root::get_routes())
        .mount("/1", day_01::get_routes())
        .mount("/4", day_04::get_routes())
        .mount("/6", day_06::get_routes())
        .mount("/7", day_07::get_routes())
        .mount("/8", day_08::get_routes())
        .mount("/11", day_11::get_routes());

    Ok(rocket.into())
}

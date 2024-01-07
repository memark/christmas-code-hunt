use shuttle_persist::{Persist, PersistInstance};
use shuttle_rocket::ShuttleRocket;
use shuttle_runtime::main;

mod day_01;
mod day_04;
mod day_06;
mod day_07;
mod day_08;
mod day_11;
mod day_12;
mod day_20;
mod day_neg_01;
mod root;

#[main]
async fn main(#[Persist] persist: PersistInstance) -> ShuttleRocket {
    let state = MyState { persist };
    let rocket = rocket::build()
        .manage(state)
        .mount("/-1", day_neg_01::get_routes())
        .mount("/", root::get_routes())
        .mount("/1", day_01::get_routes())
        .mount("/4", day_04::get_routes())
        .mount("/6", day_06::get_routes())
        .mount("/7", day_07::get_routes())
        .mount("/8", day_08::get_routes())
        .mount("/11", day_11::get_routes())
        .mount("/12", day_12::get_routes())
        .mount("/20", day_20::get_routes());
    // .data(10_i32.mebibytes());

    Ok(rocket.into())
}

struct MyState {
    persist: shuttle_persist::PersistInstance,
}

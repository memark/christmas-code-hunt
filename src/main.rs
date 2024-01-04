mod day_1;
mod day_4;
mod day_6;
mod day_7;
mod day_8;
mod day_neg_1;
mod root;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", root::get_routes())
        .mount("/-1", day_neg_1::get_routes())
        .mount("/1", day_1::get_routes())
        .mount("/4", day_4::get_routes())
        .mount("/6", day_6::get_routes())
        .mount("/7", day_7::get_routes())
        .mount("/8", day_8::get_routes());

    Ok(rocket.into())
}

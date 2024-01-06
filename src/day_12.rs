use crate::MyState;
use chrono::{DateTime, Datelike, Utc};
use itertools::Itertools;
use rocket::{get, post, routes, serde::json::Json, Route, State};
use serde::Serialize;
use ulid::Ulid;
use uuid::Uuid;

pub fn get_routes() -> Vec<Route> {
    routes![save, load, ulids, ulids_weekday]
}

#[post("/save/<string>")]
fn save(string: &str, state: &State<MyState>) {
    state.persist.save(string, Utc::now()).unwrap();
}

#[get("/load/<string>")]
fn load(string: &str, state: &State<MyState>) -> String {
    let now = Utc::now();
    let then = state.persist.load::<DateTime<Utc>>(string).unwrap();
    (now - then).num_seconds().to_string()
}

#[post("/ulids", data = "<input>")]
fn ulids(input: Json<Vec<String>>) -> Json<Vec<String>> {
    Json(
        input
            .iter()
            .map(|s| Ulid::from_string(s).unwrap())
            .map(|ulid| Uuid::from(ulid).to_string())
            .rev()
            .collect(),
    )
}

#[post("/ulids/<weekday>", data = "<input>")]
fn ulids_weekday(input: Json<Vec<String>>, weekday: u8) -> Json<UlidsWeekdayResult> {
    let ulids = input
        .iter()
        .map(|s| Ulid::from_string(s).unwrap())
        .collect_vec();

    Json(UlidsWeekdayResult {
        christmas_eve: ulids.iter().filter(|&u| gen_on_christmas_eve(u)).count(),
        weekday: ulids.iter().filter(|&u| gen_on_weekday(u, weekday)).count(),
        in_the_future: ulids.iter().filter(|&u| gen_in_future(u)).count(),
        lsb_is_1: ulids.iter().filter(|&u| lsb_is_1(u)).count(),
    })
}

fn gen_on_christmas_eve(ulid: &Ulid) -> bool {
    let datetime: DateTime<Utc> = ulid.datetime().into();
    (datetime.month(), datetime.day()) == (12, 24)
}

fn gen_on_weekday(ulid: &Ulid, weekday: u8) -> bool {
    let weekday = weekday as u32;
    let datetime: DateTime<Utc> = ulid.datetime().into();
    datetime.weekday().num_days_from_monday() == weekday
}

fn gen_in_future(ulid: &Ulid) -> bool {
    ulid.datetime() > std::time::SystemTime::now()
}

fn lsb_is_1(ulid: &Ulid) -> bool {
    ulid.0 & 1 == 1
}

#[derive(Serialize)]
struct UlidsWeekdayResult {
    #[serde(rename = "christmas eve")]
    christmas_eve: usize,
    weekday: usize,
    #[serde(rename = "in the future")]
    in_the_future: usize,
    #[serde(rename = "LSB is 1")]
    lsb_is_1: usize,
}

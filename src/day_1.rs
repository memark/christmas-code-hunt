use rocket::get;
use std::{ops::BitXor, path::PathBuf};

#[get("/<nums..>")]
pub fn nums(nums: PathBuf) -> String {
    nums.iter()
        .map(|x| x.to_str().unwrap().parse().unwrap())
        .reduce(i32::bitxor)
        .unwrap()
        .pow(3)
        .to_string()
}

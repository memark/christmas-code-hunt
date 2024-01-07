#![allow(unused_imports, unused_variables)]
use rocket::{data::ToByteUnit, fs::TempFile, post, routes, Data, Route};
use std::fs::File;
use tar::Archive;

pub fn get_routes() -> Vec<Route> {
    routes![archive_files]
}

#[post("/archive_files", data = "<file>")]
async fn archive_files(file: Data<'_>) -> String {
    // let path = file.path().unwrap();
    // let mut archive = Archive::new(File::open(path).unwrap());
    // archive.entries().unwrap().count().to_string();

    file.open(512_i32.kibibytes())
        .stream_to(tokio::io::stdout())
        .await
        .unwrap();

    0.to_string()
}
// mut file: TempFile<'_>
// async fn debug(data: Data<'_>) -> std::io::Result<()> {

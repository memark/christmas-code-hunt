use image::{io::Reader as ImageReader, GenericImageView, Rgba};
use rocket::{
    form::Form,
    fs::{NamedFile, TempFile},
    get, post, routes, FromForm, Route,
};
use std::path::{Path, PathBuf};

pub fn get_routes() -> Vec<Route> {
    routes![assets, red_pixels]
}

#[get("/assets/<path..>")]
async fn assets(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets").join(path)).await.ok()
}

#[post("/red_pixels", data = "<upload>")]
fn red_pixels(upload: Form<Upload<'_>>) -> String {
    let path = upload.image.path().unwrap();
    let image = ImageReader::open(path)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    image.pixels().filter(is_magic_red).count().to_string()
}

#[derive(FromForm)]
struct Upload<'r> {
    image: TempFile<'r>,
}

fn is_magic_red(pixel: &(u32, u32, Rgba<u8>)) -> bool {
    let (_, _, Rgba([r, g, b, _a])) = pixel;
    // Convert to u16 to avoid overflows.
    *r as u16 > *g as u16 + *b as u16
}

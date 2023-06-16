#[macro_use] extern crate rocket;
extern crate rand;

mod image;
use image::{upload_png, upload_jpeg};

use rocket::fs::{FileServer, relative, TempFile};
use rocket::form::Form;
use rocket::http::ContentType;

#[derive(FromForm)]
struct Upload<'r> {
    #[field(validate = ext(ContentType::PNG))]
    file: TempFile<'r>,
}

#[post("/upload", data = "<upload>")]
async fn upload_form(mut upload: Form<Upload<'_>>) -> std::io::Result<()> {
    upload.file.persist_to(concat!(env!("CARGO_MANIFEST_DIR"), "/upload/img.png")).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![upload_form])
        .mount("/get", FileServer::from(relative!("upload")))
        .mount("/img", routes![upload_png, upload_jpeg])
}

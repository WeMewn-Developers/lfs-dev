#[macro_use] extern crate rocket;
extern crate rand;

mod id;

mod image;
use image::{upload_png, delete_png, upload_jpeg, delete_jpeg};

mod audio;
use audio::{upload_wav, delete_wav, upload_mp3, delete_mp3, upload_ogg, delete_ogg, upload_aac, delete_aac};

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
        .mount("/img", routes![upload_png, delete_png, upload_jpeg, delete_jpeg])
        .mount("/audio", routes![upload_wav, delete_wav, upload_mp3, delete_mp3, upload_ogg, delete_ogg, upload_aac, delete_aac])
}

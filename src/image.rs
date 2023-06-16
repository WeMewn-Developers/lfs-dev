use rocket::http::{ContentType, Status};
use rocket::fs::TempFile;
use rocket::form::Form;

use std::fs::remove_file;

use crate::id::Id;

#[derive(FromForm)]
pub struct PNGUpload<'r> {
    #[field(validate = ext(ContentType::PNG))]
    file: TempFile<'r>,
}

#[post("/png", data = "<image>")]
pub async fn upload_png(mut image: Form<PNGUpload<'_>>) -> (Status, String) {
    let id = Id::new(8, "png");
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/png/", id.0.as_ref(), ".png");
    match image.file.persist_to(file_path).await {
        Ok(_) => (Status::Ok, format!("{}/{}{}", "png", id.0.as_ref().to_string(), ".png")),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[delete("/png/<id>")]
pub fn delete_png(id: Id<'_>) -> (Status, String) {
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/png", id.0.as_ref(), ".png");
    match remove_file(file_path) {
        Ok(_) => (Status::Ok, id.0.as_ref().to_string()),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[derive(FromForm)]
pub struct JPEGUpload<'r> {
    #[field(validate = ext(ContentType::JPEG))]
    file: TempFile<'r>,
}

#[post("/jpeg", data = "<image>")]
pub async fn upload_jpeg(mut image: Form<JPEGUpload<'_>>) -> (Status, String) {
    let id = Id::new(8, "jpeg");
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/jpeg/", id.0.as_ref(), ".png");
    match image.file.persist_to(file_path).await {
        Ok(_) => (Status::Ok, format!("{}/{}{}", "jpeg", id.0.as_ref().to_string(), ".jpeg")),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[delete("/jpeg/<id>")]
pub fn delete_jpeg(id: Id<'_>) -> (Status, String) {
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/jpeg/", id.0.as_ref(), ".jpeg");
    match remove_file(file_path) {
        Ok(_) => (Status::Ok, id.0.as_ref().to_string()),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}
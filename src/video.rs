use rocket::http::{ContentType, Status};
use rocket::fs::TempFile;
use rocket::form::Form;

use std::fs::remove_file;

use crate::id::Id;

#[derive(FromForm)]
pub struct MP4Upload<'r> {
    #[field(validate = ext(ContentType::MP4))]
    file: TempFile<'r>,
}

#[post("/mp4", data = "<video>")]
pub async fn upload_mp4(mut video: Form<MP4Upload<'_>>) -> (Status, String) {
    let id = Id::new(8, "mp4");
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/mp4/", id.0.as_ref(), ".mp4");
    match video.file.persist_to(file_path).await {
        Ok(_) => (Status::Ok, format!("{}/{}{}", "mp4", id.0.as_ref(), ".mp4")),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[delete("/mp4/<id>")]
pub fn delete_mp4(id: Id<'_>) -> (Status, String) {
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/mp4/", id.0.as_ref(), ".mp4");
    match remove_file(file_path) {
        Ok(_) => (Status::Ok, id.0.as_ref().to_string()),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[derive(FromForm)]
pub struct GifUpload<'r> {
    #[field(validate = ext(ContentType::GIF))]
    file: TempFile<'r>,
}

#[post("/gif", data = "<video>")]
pub async fn upload_gif(mut video: Form<GifUpload<'_>>) -> (Status, String) {
    let id = Id::new(8, "gif");
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/gif/", id.0.as_ref(), ".gif");
    match video.file.persist_to(file_path).await {
        Ok(_) => (Status::Ok, format!("{}/{}{}", "gif", id.0.as_ref(), ".gif")),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[delete("/gif/<id>")]
pub fn delete_gif(id: Id<'_>) -> (Status, String) {
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/gif/", id.0.as_ref(), ".gif");
    match remove_file(file_path) {
        Ok(_) => (Status::Ok, id.0.as_ref().to_string()),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}
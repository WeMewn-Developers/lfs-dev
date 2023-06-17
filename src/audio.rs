use rocket::http::{ContentType, Status};
use rocket::fs::TempFile;
use rocket::form::Form;

use std::fs::remove_file;

use crate::id::Id;

#[derive(FromForm)]
pub struct WavUploadBin<'r> {
    #[field(validate = ext(ContentType::Binary))]
    file: TempFile<'r>,
}

#[post("/wav", data = "<data>")]
pub async fn upload_wav(mut data: Form<WavUploadBin<'_>>) -> (Status, String) {
    let id = Id::new(8, "wav");
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/wav/", id.0.as_ref(), ".wav");
    match data.file.persist_to(file_path).await {
        Ok(_) => (Status::Ok, format!("{}/{}{}", "wav", id.0.as_ref(), ".wav")),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[delete("/wav/<id>")]
pub fn delete_wav(id: Id<'_>) -> (Status, String) {
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/wav/", id.0.as_ref(), ".wav");
    match remove_file(file_path) {
        Ok(_) => (Status::Ok, id.0.as_ref().to_string()),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[derive(FromForm)]
pub struct Mp3Upload<'r> {
    #[field(validate = ext(ContentType::Binary))]
    file: TempFile<'r>,
}

#[post("/mp3", data = "<data>")]
pub async fn upload_mp3(mut data: Form<Mp3Upload<'_>>) -> (Status, String) {
    let id = Id::new(8, "mp3");
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/mp3/", id.0.as_ref(), ".mp3");
    match data.file.persist_to(file_path).await {
        Ok(_) => (Status::Ok, format!("{}/{}{}", "mp3", id.0.as_ref(), ".mp3")),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[delete("/mp3/<id>")]
pub fn delete_mp3(id: Id<'_>) -> (Status, String) {
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/mp3/", id.0.as_ref(), ".mp3");
    match remove_file(file_path) {
        Ok(_) => (Status::Ok, id.0.as_ref().to_string()),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[derive(FromForm)]
pub struct OggUpload<'r> {
    #[field(validate = ext(ContentType::Binary))]
    file: TempFile<'r>,
}

#[post("/ogg", data = "<data>")]
pub async fn upload_ogg(mut data: Form<OggUpload<'_>>) -> (Status, String) {
    let id = Id::new(8, "ogg");
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/ogg/", id.0.as_ref(), ".ogg");
    match data.file.persist_to(file_path).await {
        Ok(_) => (Status::Ok, format!("{}/{}{}", "ogg", id.0.as_ref(), ".ogg")),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[delete("/ogg/<id>")]
pub fn delete_ogg(id: Id<'_>) -> (Status, String) {
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/ogg/", id.0.as_ref(), ".ogg");
    match remove_file(file_path) {
        Ok(_) => (Status::Ok, id.0.as_ref().to_string()),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[derive(FromForm)]
pub struct AacUpload<'r> {
    #[field(validate = ext(ContentType::Binary))]
    file: TempFile<'r>,
}

#[post("/aac", data = "<data>")]
pub async fn upload_aac(mut data: Form<AacUpload<'_>>) -> (Status, String) {
    let id = Id::new(8, "aac");
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/aac/", id.0.as_ref(), ".aac");
    match data.file.persist_to(file_path).await {
        Ok(_) => (Status::Ok, format!("{}/{}{}", "aac", id.0.as_ref(), ".aac")),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}

#[delete("/aac/<id>")]
pub fn delete_aac(id: Id<'_>) -> (Status, String) {
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/aac/", id.0.as_ref(), ".aac");
    match remove_file(file_path) {
        Ok(_) => (Status::Ok, id.0.as_ref().to_string()),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}
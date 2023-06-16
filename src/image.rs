use rocket::http::{ContentType, Status};
use rocket::fs::TempFile;
use rocket::form::Form;

mod image_id;
use image_id::ImageId;

#[derive(FromForm)]
pub struct PNGUpload<'r> {
    #[field(validate = ext(ContentType::PNG))]
    file: TempFile<'r>,
}

#[post("/png", data = "<image>")]
pub async fn upload_png(mut image: Form<PNGUpload<'_>>) -> (Status, String) {
    let id = ImageId::new(8, "png");
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/png/", id.0.as_ref(), ".png");
    match image.file.persist_to(file_path).await {
        Ok(_) => (Status::Ok, String::from(format!("{}/{}{}", "png", id.0.as_ref().to_string(), ".png"))),
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
    let id = ImageId::new(8, "jpeg");
    let file_path = format!("{}{}{}{}", env!("CARGO_MANIFEST_DIR"), "/upload/jpeg/", id.0.as_ref(), ".png");
    match image.file.persist_to(file_path).await {
        Ok(_) => (Status::Ok, String::from(format!("{}/{}{}", "jpeg", id.0.as_ref().to_string(), ".jpeg"))),
        Err(err) => (Status::BadRequest, err.to_string()),
    }
}
mod image_id;

use std::path::Path;

use rocket::tokio::fs::File;
use rocket::Data;

use image_id::ImageId;

#[get("/<id>")]
async fn get_image(id: ImageId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}
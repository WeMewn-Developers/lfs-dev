use std::borrow::Cow;
use std::path::{Path, PathBuf};

use rocket::request::FromParam;

use rand::{Rng};

pub struct ImageId<'a>(Cow<'a, str>);

#[derive(Debug)]
pub enum ImageError {
    InvalidId,
    ImageNotFound,
}

impl ImageId<'_> {
    /// Create a new Image ID
    pub fn new(size: usize) -> ImageId<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        ImageId(Cow::Owned(id))
    }

    /// Return the path to the Image
    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "images");
        Path::new(root).join(self.0.as_ref())
    }
}

impl<'a> FromParam<'a> for ImageId<'a> {
    type Error = ImageError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        if !param.chars().all(|c| c.is_ascii_alphanumeric()) {
            Err(ImageError::InvalidId)
        } else if !ImageId(param.into()).file_path().exists() {
            Err(ImageError::ImageNotFound)
        } else {
            Ok(ImageId(param.into()))
        }
    }
}


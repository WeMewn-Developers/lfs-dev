use std::borrow::Cow;
use std::path::Path;

use rocket::request::FromParam;

use rand::Rng;

pub struct Id<'a>(pub Cow<'a, str>);

impl Id<'_> {
    /// Generate a new unique Image Id
    pub fn new(size: usize, filetype: &str) -> Id<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let id: String;
        let mut rng = rand::thread_rng();
        loop {
            let mut potential_id = String::with_capacity(size);
            for _ in 0..size {
                potential_id.push(BASE62[rng.gen::<usize>() % 62] as char);
            }

            if !Path::new(&format!("{}/{}/{}.{}", env!("CARGO_MANIFEST_DIR"), filetype, potential_id, filetype)).exists() {
                id = potential_id;
                break;
            }
        }

        Id(Cow::Owned(id))
    }
}

impl<'a> FromParam<'a> for Id<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        if !param.chars().all(|c| c.is_ascii_alphanumeric()) {
            Err(param)
        } else {
            Ok(Id(param.into()))
        }
    }
}
use hmac::{Hmac, Mac};
use jwt::{Header, Token, VerifyWithKey};
use rocket::http::Status;
use sha2::Sha384;
use std::collections::BTreeMap;
use std::io::Cursor;

use rocket::request::Request;
use rocket::response::Response;
use rocket::fairing::{Fairing, Info, Kind};

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use chrono::{DateTime, Utc};

const AUTH_KEY: &'static [u8] = b"DINOSAUR";
const DATABASE_URL: &'static str = "mysql://root:root@db:3306/test";

#[derive(Queryable)]
pub struct AudioTokenHeader {
    iss: DateTime<Utc>,
    exp: DateTime<Utc>,
}

#[derive(Queryable)]
pub struct AudioTokenBody {
    format: String,
    username: String,
    file_name: String,
    file_size: u64,
}

#[derive(Queryable)]
pub struct AudioToken {
    header: AudioTokenHeader,
    body: AudioTokenBody,
}

struct ResponseJWTGuard {

}

#[rocket::async_trait]
impl Fairing for ResponseJWTGuard {
    fn info(&self) -> Info {
        Info {
            name: "Image Get/Post/Delete Guard",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let headers = request.headers();

        let token_str = match headers.get_one("Authentication") {
            Some(token) => token,
            None => {
                return edit_failed_response(response);
            }
        };

        let key: Hmac<Sha384> = Hmac::new_from_slice(AUTH_KEY).unwrap();

        let token: Token<Header, BTreeMap<String, String>, _> = match token_str.verify_with_key(&key) {
            Ok(token) => token,
            Err(_err) => {
                return edit_failed_response(response);
            }
        };

        let database_connection = MysqlConnection::establish(&DATABASE_URL).unwrap();

        let header = token.header();
        // TODO: Get the expected header data from the database

        let body = token.claims();
        // TODO: Get the expected body data from the database
    }
}

fn edit_failed_response<'r>(response: &mut Response<'r>) {
    response.set_status(Status::Unauthorized);
    let body = "Unauthorized Access";
    response.set_sized_body(body.len(), Cursor::new(body));
    return;
}
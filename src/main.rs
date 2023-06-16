#[macro_use] extern crate rocket;
extern crate rand;

mod image;

#[get("/")]
fn index() -> &'static str {
    "Hello"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}

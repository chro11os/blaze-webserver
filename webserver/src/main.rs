mod error_handling;
mod progress_bar;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};


#[macro_use]
extern crate rocket;

#[get("/")] // this routes the server to the index homepage: "/"
fn index() -> &'static str {
    "OK"
}

#[launch] // launches and builds the server
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}


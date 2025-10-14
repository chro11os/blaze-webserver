#[macro_use] extern crate rocket;

// 1. Declare the `handler` module. This tells Rust to look for `handler.rs`.
mod handler;

// 2. Import the public functions from the `handler` module.
use handler::{create_user, get_user};

#[launch]
fn rocket() -> _ {
    // 3. The routes macro works the same because the functions are now in scope.
    rocket::build().mount("/", routes![create_user, get_user])
}
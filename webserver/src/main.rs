#[macro_use] extern crate rocket;

// Import sqlx
use rocket::State;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use rocket::fs::FileServer;
use rocket::response::Responder;
use rocket::Request;
use rocket::http::Status;
use rocket::local::blocking::Client;
mod handler;

use handler::{create_user, get_user};

// Define a simple error type for our application
#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        // We'll log the detailed error to the console for debugging
        eprintln!("An error occurred: {:?}", self);

        // For the client, we'll just return a generic 500 error
        // to avoid leaking implementation details.
        Err(Status::InternalServerError)
    }
}


// The database pool will be managed by Rocket
pub struct DbState {
    pub pool: SqlitePool,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenvy::dotenv().ok();
    // Define the database URL
    let db_url = "sqlite:database.db";

    // Create a connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Failed to create database pool.");

    println!("🚀 Starting server...");

    // Launch the Rocket server
    let rocket = rocket::build()
        .manage(DbState { pool }) // Add the pool to Rocket's managed state
        .mount("/api", routes![create_user, get_user])
        .mount("/", FileServer::from("../static"))
        .launch()
        .await?;

    // Launch Testing
    let client = Client::tracked(rocket).unwrap();
    let req = client.get("/api");
    let response = req.dispatch();

    Ok(())
}
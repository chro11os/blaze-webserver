use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

// The struct and its fields must be public
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

// The handler functions must be public
#[post("/users", format = "json", data = "<user>")]
pub fn create_user(user: Json<User>) -> Json<User> {
    println!("Created user: {} with ID: {}", user.name, user.id);
    user
}

#[get("/users/<id>")]
pub fn get_user(id: u32) -> Option<Json<User>> {
    if id == 1 {
        Some(Json(User {
            id: 1,
            name: "Neil Guzman".to_string(),
        }))
    } else {
        None
    }
}
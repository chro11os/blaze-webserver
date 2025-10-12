#[post("/users", format = "json", data="<user>")]
fn create_user(user: Json<User>) {
    println("Created User: {}", user.name)
}
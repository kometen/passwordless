mod client;
mod models;
mod utils;

#[macro_use]
extern crate rocket;

use models::{PasswordOptions, Pwd};
use rocket::serde::json::Json;
use utils::generate_passwords;

#[get("/pwd")]
fn get_pwd() -> Json<Vec<Pwd>> {
    generate_passwords(&PasswordOptions::new(5, 20))
}

#[get("/pwd/<count>")]
fn pwd_count(count: usize) -> Json<Vec<Pwd>> {
    let c = count.min(31);
    let password_options = PasswordOptions::new(c, 20);
    generate_passwords(&password_options)
}

#[post("/pwd", data = "<password_options>")]
fn post_pwd(password_options: Json<PasswordOptions>) -> Json<Vec<Pwd>> {
    generate_passwords(&password_options)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_pwd, pwd_count, post_pwd])
}

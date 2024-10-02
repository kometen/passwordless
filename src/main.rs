mod client;
mod models;
mod tests;
mod utils;

#[macro_use]
extern crate rocket;
extern crate dotenv;

use client::PasswordlessClient;
use dotenv::dotenv;
use models::{PasswordOptions, Pwd, RegisterRequest, SignInVerifyRequest};
use rocket::{fs::FileServer, serde::json::Json, State};
use rocket_dyn_templates::{context, Template};
use serde_json::{json, Value};
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

#[post("/register", format = "json", data = "<data>")]
pub async fn register(client: &State<PasswordlessClient>, data: Json<Value>) -> Value {
    let register_options = RegisterRequest {
        user_id: data.get("userId").unwrap().to_string(),
        username: data.get("username").unwrap().to_string(),
        display_name: data.get("username").unwrap().to_string(),
    };

    let token = client.register_token(&register_options).await.unwrap();
    json!(token)
}

#[post("/login", format = "json", data = "<data>")]
pub async fn login(client: &State<PasswordlessClient>, data: Json<SignInVerifyRequest>) -> Value {
    let response = client.sign_in(&data).await.unwrap();
    json!(response)
}

#[get("/")]
pub fn index() -> Template {
    let passwordless_api_key =
        &std::env::var("PASSWORDLESS_API_KEY").expect("PASSWORDLESS_API_KEY must be set.");
    let passwordless_api_url =
        &std::env::var("PASSWORDLESS_API_URL").expect("PASSWORDLESS_API_URL must be set.");
    Template::render(
        "index",
        context! { passwordless_api_url: passwordless_api_url, passwordless_api_key: passwordless_api_key  },
    )
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let client = PasswordlessClient::new(
        &std::env::var("PASSWORDLESS_API_SECRET").expect("PASSWORDLESS_API_SECRET must be set."),
        &std::env::var("PASSWORDLESS_API_URL").expect("PASSWORDLESS_API_URL must be set."),
    );

    rocket::build()
        .mount("/", routes![get_pwd, index, pwd_count, post_pwd])
        .mount("/passwordless/api", routes![register, login])
        .mount("/static", FileServer::from("src/static"))
        .manage(client)
        .attach(Template::fairing())
}

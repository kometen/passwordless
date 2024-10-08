use crate::models::{PasswordOptions, Pwd};
use passwords::{analyzer, scorer, PasswordGenerator};
use rocket::serde::json::Json;

pub fn generate_passwords(password_options: &PasswordOptions) -> Json<Vec<Pwd>> {
    let c = password_options.count.unwrap_or(5);
    let pwd_length = password_options.length.unwrap_or(20);
    let option_numbers = password_options.numbers.unwrap_or(true);
    let option_lowercase_letters = password_options.lowercase_letters.unwrap_or(true);
    let option_uppercase_letters = password_options.uppercase_letters.unwrap_or(true);
    let option_symbols = password_options.symbols.unwrap_or(false);
    let option_spaces = password_options.spaces.unwrap_or(true);
    let option_exclude_similar_characters =
        password_options.exclude_similar_characters.unwrap_or(false);
    let option_strict = password_options.strict.unwrap_or(false);

    let pg = PasswordGenerator {
        length: pwd_length,
        numbers: option_numbers || option_strict,
        lowercase_letters: option_lowercase_letters || option_strict,
        uppercase_letters: option_uppercase_letters || option_strict,
        symbols: option_symbols || option_strict,
        spaces: option_spaces || option_strict,
        exclude_similar_characters: option_exclude_similar_characters,
        strict: option_strict,
    };

    let pwd: Vec<Pwd> = pg
        .generate(c)
        .unwrap()
        .into_iter()
        .map(|x| Pwd {
            password: x.clone(),
            score: scorer::score(&analyzer::analyze(&x)).ceil() as u8,
        })
        .collect();

    Json(pwd)
}

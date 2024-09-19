use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Default)]
pub struct PasswordOptions {
    pub count: Option<usize>,
    pub length: Option<usize>,
    pub numbers: Option<bool>,
    pub lowercase_letters: Option<bool>,
    pub uppercase_letters: Option<bool>,
    pub symbols: Option<bool>,
    pub spaces: Option<bool>,
    pub exclude_similar_characters: Option<bool>,
    pub strict: Option<bool>,
}

impl PasswordOptions {
    pub fn new(count: usize, length: usize) -> PasswordOptions {
        PasswordOptions {
            count: Some(count),
            length: Some(length),
            ..Default::default()
        }
    }
}

#[derive(Serialize)]
pub struct Pwd {
    pub password: String,
    pub score: u8,
}

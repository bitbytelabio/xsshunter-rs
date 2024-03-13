use bcrypt::{hash, DEFAULT_COST};
use chrono::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;
use tracing::info;

pub fn get_secure_random_string(length: usize) -> String {
    let secure_random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    secure_random_string
}

pub fn get_hashed_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let bcrypt_rounds: u32 = env::var("BCRYPT_ROUNDS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .unwrap_or(DEFAULT_COST);

    hash(password, bcrypt_rounds)
}

#[test]
fn returns_string_of_specified_length() {
    let length = 10;
    let result = get_secure_random_string(length);
    assert_eq!(result.len(), length);
}

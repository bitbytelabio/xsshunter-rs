use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn get_secure_random_string(length: usize) -> String {
    let secure_random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    secure_random_string
}

// Returns a string of the specified length
#[test]
fn returns_string_of_specified_length() {
    let length = 10;
    let result = get_secure_random_string(length);
    assert_eq!(result.len(), length);
}

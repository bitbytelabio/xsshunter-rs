use regex::Regex;
use std::env;

lazy_static::lazy_static!(
    static ref SCREENSHOTS_DIR: String = env::var("SCREENSHOTS_DIR").unwrap_or_else(|_| "screenshots".to_string());
    static ref SCREENSHOT_FILENAME_REGEX: Regex =
        Regex::new(r"^[0-9A-F]{8}-[0-9A-F]{4}-4[0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12}\.png$")
            .expect("Unable to compile regex");
);

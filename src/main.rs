#![allow(unused)]

use regex::Regex;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod app;
mod config;
mod db;
mod errors;
mod logs;
mod models;
mod utils;

const ADMIN_PASSWORD_SETTINGS_KEY: &'static str = "ADMIN_PASSWORD";
const API_BASE_PATH: &'static str = "api/v1";
const CORRELATION_API_SECRET_SETTINGS_KEY: &'static str = "CORRELATION_API_KEY";
const CHAINLOAD_URI_SETTINGS_KEY: &'static str = "CHAINLOAD_URI";
const PAGES_TO_COLLECT_SETTINGS_KEY: &'static str = "PAGES_TO_COLLECT";
const SEND_ALERT_EMAILS_KEY: &'static str = "SEND_ALERT_EMAILS";
const SESSION_SECRET_KEY: &'static str = "SESSION_SECRET";
const CSRF_HEADER_NAME: &'static str = "X-CSRF-Buster";

lazy_static::lazy_static!(
     static ref CONFIG: config::Settings = config::Settings
        ::new()
        .expect("Failed to load config, CONFIG_DIR must be set");

    static ref SCREENSHOTS_DIR: String = env::var("SCREENSHOTS_DIR").unwrap_or_else(|_| "screenshots".to_string());

    static ref SCREENSHOT_FILENAME_REGEX: Regex = Regex::new(r"^[0-9A-F]{8}-[0-9A-F]{4}-4[0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12}\.png$")
        .expect("Failed to create screenshot filename regex");
);

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // logs::start_logging();
    app::run().await;
}

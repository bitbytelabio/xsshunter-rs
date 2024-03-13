#![allow(unused)]

use sqlx::postgres::PgPoolOptions;
use std::env;

mod app;
mod config;
mod db;
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

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
    db::initialize_database(&pool).await;
}

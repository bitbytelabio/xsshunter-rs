use crate::{db, errors::Result};
use axum::http::Method;
use axum::routing;
use axum::routing::post;
use axum::Router;
use regex::Regex;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use self::handlers::page_callback_handler;

mod handlers;
mod routes;

lazy_static::lazy_static!(
    static ref SCREENSHOTS_DIR: String = env::var("SCREENSHOTS_DIR").unwrap_or_else(|_| "screenshots".to_string());
    static ref SCREENSHOT_FILENAME_REGEX: Regex = Regex::new(r"^[0-9A-F]{8}-[0-9A-F]{4}-4[0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12}\.png$")
        .expect("Failed to create screenshot filename regex");
);

pub async fn run() {
    let address = env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let listener = match TcpListener::bind(address).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address: {:?}", e);
            std::process::exit(1);
        }
    };

    let pool = match db::create_connection_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create connection pool: {:?}", e);
            std::process::exit(1);
        }
    };

    db::initialize_database(&pool).await;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .merge(routes::create_routes())
        // .merge(routes::create_api_routes())
        .layer(cors)
        .with_state(pool);

    match axum::serve(listener, app).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error running server: {:?}", e);
            std::process::exit(1);
        }
    }
}

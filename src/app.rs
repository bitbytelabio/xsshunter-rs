use crate::{db, errors::Result};
use axum::routing;
use axum::Router;
use regex::Regex;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;

mod handlers;
mod routes;

pub struct App {
    pub screenshots_dir: String,
    pub screenshot_filename_regex: Regex,
}

impl App {
    pub async fn new() -> Result<Self> {
        let screenshots_dir =
            env::var("SCREENSHOTS_DIR").unwrap_or_else(|_| "screenshots".to_string());
        let screenshot_filename_regex = Regex::new(
            r"^[0-9A-F]{8}-[0-9A-F]{4}-4[0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12}\.png$",
        )?;

        Ok(Self {
            screenshots_dir,
            screenshot_filename_regex,
        })
    }
}

pub async fn run() {
    // let mut app = App::new().await.expect("Failed to create app");

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

    let router = Router::new()
        .route("/hello", routing::get(hello_world))
        .with_state(pool.clone());

    match axum::serve(listener, router).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error running server: {:?}", e);
            std::process::exit(1);
        }
    }
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

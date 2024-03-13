use crate::{db, errors::Result};
use axum::Router;
use regex::Regex;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;

pub struct App {
    pub screenshots_dir: String,
    pub screenshot_filename_regex: Regex,
    listener: TcpListener,
    router: Router,
    pool: sqlx::PgPool,
}

impl App {
    pub async fn new() -> Result<Self> {
        let address = env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
        let listener = TcpListener::bind(address).await?;
        let screenshots_dir =
            env::var("SCREENSHOTS_DIR").unwrap_or_else(|_| "screenshots".to_string());
        let screenshot_filename_regex = Regex::new(
            r"^[0-9A-F]{8}-[0-9A-F]{4}-4[0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12}\.png$",
        )?;

        let pool = db::create_connection_pool().await?;

        db::initialize_database(&pool).await;

        Ok(Self {
            screenshots_dir,
            screenshot_filename_regex,
            listener,
            router: Router::new(),
            pool,
        })
    }

    pub async fn run(self) {
        let router = self.router;
        let listener = self.listener;

        axum::serve(listener, router)
            .await
            .expect("Server failed to start");
    }
}

use axum::Router;
use regex::Regex;
use std::env;
use tokio::net::TcpListener;

pub struct App {
    pub screenshots_dir: String,
    pub screenshot_filename_regex: Regex,
    listener: TcpListener,
}

impl App {
    pub fn new(address: String, port: u16) -> Self {
        todo!()
    }

    pub async fn run(self) {
        todo!()
    }
}

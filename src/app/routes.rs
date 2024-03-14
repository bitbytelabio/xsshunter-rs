use super::handlers::hello_handler;
use axum::{response::Html, routing::get, Router};

pub fn create_routes() -> Router {
    let app = Router::new().route("/", get(hello_handler));
    app
}

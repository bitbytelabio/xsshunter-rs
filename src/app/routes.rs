use super::handlers::{
    health_check_handler, js_callback_handler, page_callback_handler, payload_handler,
    screenshot_handler,
};
use crate::API_BASE_PATH;
use axum::{
    response::Html,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

pub fn create_routes() -> Router<PgPool> {
    let routes = Router::new()
        .route("/page_callback", post(page_callback_handler))
        .route("/js_callback", post(js_callback_handler))
        .route(
            "/screenshots/:screenshot_file_name",
            get(screenshot_handler),
        )
        .route("/health", get(health_check_handler))
        .route("/", get(payload_handler))
        .route("/:probe_id", get(payload_handler));
    routes
}

pub fn create_api_routes() -> Router {
    let api = Router::new()
        .route(
            "/admin",
            get(|| async { Html("Correlation API".to_string()) }),
        )
        .route(
            "/login",
            get(|| async { Html("Correlation API".to_string()) }),
        )
        .route(
            "/auth-check",
            get(|| async { Html("Correlation API".to_string()) }),
        )
        .route(
            "/payloadfires",
            get(|| async { Html("Correlation API".to_string()) }),
        );
    let routes = Router::new().nest(API_BASE_PATH, api);
    routes
}

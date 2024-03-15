use std::time::Duration;

use super::handlers::{
    health_check_handler, image_callback_handler, js_callback_handler, page_callback_handler,
    payload_handler, screenshot_handler,
};
use crate::API_BASE_PATH;
use axum::{
    extract::DefaultBodyLimit,
    http::{HeaderName, Method},
    response::Html,
    routing::{delete, get, post, put},
    Router,
};
use axum_extra::headers::ContentType;
use sqlx::PgPool;
use tower_http::cors::{Any, Cors, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;

pub fn create_routes() -> Router<PgPool> {
    let callback_cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers([
            HeaderName::from_static("Content-Type"),
            HeaderName::from_static("X-Requested-With"),
        ])
        .max_age(Duration::from_secs(86400));

    let callback_router = Router::new()
        .route("/js_callback", get(test).post(js_callback_handler))
        .route(
            "/page_callback",
            post(page_callback_handler).post(image_callback_handler),
        )
        .layer(callback_cors)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250mb */
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let routes = Router::new()
        .merge(callback_router)
        .route(
            "/screenshots/:screenshot_file_name",
            get(screenshot_handler),
        )
        .route("/health", get(health_check_handler))
        .route("/", get(payload_handler))
        .route("/:probe_id", get(payload_handler));
    routes
}

async fn test() {}

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

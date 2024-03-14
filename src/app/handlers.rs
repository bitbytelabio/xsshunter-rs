use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct CollectedPagesCallbackArgs {
    pub uri: String,
    pub html: String,
}

#[derive(Debug, Serialize)]
pub struct Status {
    pub status: String,
}

pub async fn page_callback_handler(
    State(pool): State<PgPool>,
    Json(body): Json<CollectedPagesCallbackArgs>,
) -> Result<Json<Status>, (StatusCode, String)> {
    let mut response = Response::new(Json(Status {
        status: "OK".into(),
    }));

    response
        .headers_mut()
        .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        "POST, OPTIONS".parse().unwrap(),
    );
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        "Content-Type, X-Requested-With".parse().unwrap(),
    );
    response
        .headers_mut()
        .insert(header::ACCESS_CONTROL_MAX_AGE, "86400".parse().unwrap());

    Ok(Json(Status {
        status: "OK".into(),
    }))
}

pub async fn js_callback_handler() -> &'static str {
    "JS callback"
}

pub async fn screenshot_handler() -> &'static str {
    "Screenshot"
}

pub async fn health_check_handler() -> &'static str {
    "Health check"
}

pub async fn payload_handler() -> &'static str {
    "Payload"
}

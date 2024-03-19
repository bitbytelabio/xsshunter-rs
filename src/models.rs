use axum::{
    body::Bytes,
    extract::{FromRequest, Multipart},
    BoxError,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use uuid::Uuid;

#[derive(Debug, FromRow, PartialEq, Clone, Serialize)]
pub struct Setting {
    pub id: Uuid,
    pub key: String,
    pub value: String,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize)]
pub struct CollectedPage {
    pub id: Uuid,
    pub uri: String,
    pub html: String,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize)]
pub struct InjectionRequest {
    pub id: Uuid,
    pub request: String,
    pub injection_key: String,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize)]
pub struct PayloadFireResult {
    pub id: Uuid,
    pub url: String,
    pub ip_address: String,
    pub referrer: String,
    pub user_agent: String,
    pub cookie: String,
    pub title: String,
    pub dom: String,
    pub text: String,
    pub origin: String,
    pub screenshot_id: String,
    pub was_iframe: bool,
    pub browser_timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CollectedPagesCallbackArgs {
    pub uri: String,
    pub html: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JSCallbackArgs {
    pub uri: String,
    pub cookies: String,
    pub referrer: String,
    pub user_agent: String,
    pub browser_time: String,
    pub probe_uid: String,
    pub origin: String,
    pub injection_key: String,
    pub title: String,
    pub text: String,
    pub was_iframe: bool,
    pub dom: String,
    pub screenshot: String,
}

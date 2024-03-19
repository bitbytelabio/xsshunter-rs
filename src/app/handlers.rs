use std::fs;

use crate::{
    db::collected_pages,
    models::{CollectedPagesCallbackArgs, JSCallbackArgs},
    SCREENSHOTS_DIR,
};
use axum::{
    body,
    extract::{multipart, Multipart, Path, State},
    http::{header, HeaderMap, HeaderValue, Response, StatusCode},
    Json,
};
use axum_macros::debug_handler;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::io::Write;
use std::path::Path as FilePath;
use uuid::Uuid;

pub async fn page_callback_handler(
    State(pool): State<PgPool>,
    Json(body): Json<CollectedPagesCallbackArgs>,
) -> Result<String, (StatusCode, String)> {
    collected_pages::create(&pool, &body.uri, &body.html)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create collected page: {:?}", e),
            )
        })?;

    Ok("OK".to_string())
}

#[debug_handler]
pub async fn js_callback_handler(
    State(pool): State<PgPool>,
    Json(body): Json<JSCallbackArgs>,
) -> Result<String, (StatusCode, String)> {
    // let multer_temp_image_path =
    let payload_fire_id = Uuid::new_v4();

    Ok("OK".to_string())
}

#[debug_handler]
pub async fn image_callback_handler(
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> Result<String, (StatusCode, String)> {
    let payload_fire_image_id = Uuid::new_v4();
    let payload_fire_image_filename = format!(
        "${}/${payload_fire_image_id}.png.gz",
        SCREENSHOTS_DIR.to_string()
    );

    // let mut s3_client = Minio::new("localhost:9000", "minioadmin", "minioadmin").unwrap();
    // s3_client = s3_client.with_secure(false);

    while let Some(mut field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to load image: {:?}", e),
        )
    })? {
        let mut data = Vec::new();
        while let Some(chunk) = field.chunk().await.map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to read chunk: {:?}", e),
            )
        })? {
            data.extend_from_slice(&chunk);
        }

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&data).unwrap();
        let compressed_data = encoder.finish().unwrap();

        // s3_client
        //     .put_object("my-bucket", &payload_fire_image_filename, &compressed_data)
        //     .await
        //     .unwrap();
    }

    Ok("OK".to_string())
}

pub async fn screenshot_handler(Path(screenshot_file_name): Path<String>) -> &'static str {
    dbg!(screenshot_file_name);
    "Screenshot"
}

pub async fn health_check_handler(headers: HeaderMap) -> &'static str {
    dbg!(headers);
    "Health check"
}

pub async fn payload_handler(Path(probe_id): Path<String>) -> &'static str {
    "Payload"
}

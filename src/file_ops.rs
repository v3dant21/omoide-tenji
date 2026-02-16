use axum::{
    extract::{Multipart, Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use std::io::{Cursor, Write};
use uuid::Uuid;
use zip::write::FileOptions;
use zip::ZipWriter;

use crate::s3::{download_from_s3, list_gallery_images, upload_to_s3};
use crate::AppState;

pub async fn upload_gallery(
    State(state): State<AppState>,
    Path(id): Path<String>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("\n Upload started for gallery: {id}");
    let mut uploaded_urls: Vec<String> = Vec::new();
    let mut count: usize = 0;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Multipart error: {e}")))?
    {
        let field_name = field.name().unwrap_or("").to_string();
        if field_name != "image" {
            continue;
        }

        let original_name = field.file_name().unwrap_or("unknown").to_string();
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        // Derive extension from content type
        let ext = match content_type.as_str() {
            "image/png" => "png",
            "image/gif" => "gif",
            "image/webp" => "webp",
            _ => "jpg",
        };

        let data = field
            .bytes()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Read error: {e}")))?
            .to_vec();

        if data.is_empty() {
            continue;
        }

        count += 1;
        let size_kb = data.len() as f64 / 1024.0;
        println!("  📷 [{count}] Received: {original_name} ({size_kb:.1} KB, {content_type})");

        let file_id = Uuid::new_v4();
        let key = format!("galleries/{id}/{file_id}.{ext}");
        println!("  🔑 [{count}] S3 key: {key}");

        let url = upload_to_s3(&state.s3_client, &state.bucket, &key, data, &content_type)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to upload {key}: {e}")))?;

        println!("  ✅ [{count}] Uploaded to S3: {key}");
        uploaded_urls.push(url);
    }

    if uploaded_urls.is_empty() {
        println!("  ⚠️  No images provided");
        return Err((
            StatusCode::BAD_REQUEST,
            "No images provided".to_string(),
        ));
    }

    println!("Upload complete: {count} image(s) uploaded to gallery {id}\n");

    Ok(Json(serde_json::json!({
        "gallery_id": id,
        "images": uploaded_urls,
    })))
}

pub async fn download_gallery(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let keys = list_gallery_images(&state.s3_client, &state.bucket, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    if keys.is_empty() {
        return Err((StatusCode::NOT_FOUND, "Gallery is empty or not found".to_string()));
    }

    let buf = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(buf);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for key in &keys {
        let data = download_from_s3(&state.s3_client, &state.bucket, key)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // Use only the filename portion of the key
        let filename = key.rsplit('/').next().unwrap_or(key);

        zip.start_file(filename, options)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("ZIP error: {e}")))?;
        zip.write_all(&data)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("ZIP write error: {e}")))?;
    }

    let result = zip
        .finish()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("ZIP finish error: {e}")))?;

    let zip_bytes = result.into_inner();

    let headers = [
        (header::CONTENT_TYPE, "application/zip".to_string()),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=gallery.zip".to_string(),
        ),
    ];

    Ok((headers, zip_bytes))
}

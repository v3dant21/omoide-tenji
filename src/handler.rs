use axum::{
    extract::State,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::s3::list_gallery_images;
use crate::AppState;

pub async fn health_check_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

pub async fn create_gallery() -> impl IntoResponse {
    let gallery_id = Uuid::new_v4().to_string();
    let share_url = format!("/g/{gallery_id}");

    Json(serde_json::json!({
        "gallery_id": gallery_id,
        "share_url": share_url,
    }))
}

pub async fn get_gallery(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let keys = list_gallery_images(&state.s3_client, &state.bucket, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let region = std::env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string());
    let image_urls: Vec<String> = keys
        .iter()
        .map(|key| format!("https://{}.s3.{}.amazonaws.com/{}", state.bucket, region, key))
        .collect();

    Ok(Json(serde_json::json!({
        "gallery_id": id,
        "images": image_urls,
    })))
}

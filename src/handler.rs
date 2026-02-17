use axum::{
    extract::State,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;


use crate::s3::{get_public_url, list_gallery_images};
use crate::AppState;

pub async fn health_check_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

pub async fn create_gallery() -> impl IntoResponse {
    let gallery_id = uuid::Uuid::new_v4().to_string();
    let share_url = format!("/g/{gallery_id}");

    Json(serde_json::json!({
        "gallery_id": gallery_id,
        "share_url": share_url
    }))
}

#[derive(Serialize)]
pub struct GalleryResponse {
    pub gallery_id: String,
    pub images: Vec<String>,
}

pub async fn get_gallery(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GalleryResponse>, (StatusCode, String)> {
    let keys = list_gallery_images(&state.s3_client, &state.bucket, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let region = std::env::var("AWS_REGION").unwrap_or_else(|_| "eu-north-1".to_string());
    let images: Vec<String> = keys
        .iter()
        .map(|key| get_public_url(&state.bucket, key, &region))
        .collect();

    Ok(Json(GalleryResponse {
        gallery_id: id,
        images,
    }))
}

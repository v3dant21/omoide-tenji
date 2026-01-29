use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use base64::Engine;
use serde::Serialize;

pub async fn health_check_handler()
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let json_response = serde_json::json!({
        "status": "ok"
    });

    Ok((StatusCode::OK, Json(json_response)))
}
#[derive(Serialize)]

pub struct UploadPreviewResponse {
    pub images: Vec<String>,
}

pub async fn image_handler(mut multipart: Multipart) -> Result<impl IntoResponse, StatusCode> {
    let mut images_out: Vec<String> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let name = field.name().unwrap_or("");

        if name != "image" {
            continue;
        }

        // ✅ Make it owned String so it doesn't borrow `field`
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        // ✅ Now it can be moved safely
        let data = field
            .bytes()
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .to_vec();

        if data.is_empty() {
            continue;
        }

        let b64 = base64::engine::general_purpose::STANDARD.encode(&data);
        let data_url = format!("data:{};base64,{}", content_type, b64);

        images_out.push(data_url);
    }

    if images_out.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(Json(UploadPreviewResponse { images: images_out }))
}

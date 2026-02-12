use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use crate::file_ops::{download_gallery, upload_gallery};
use crate::handler::{create_gallery, get_gallery, health_check_handler};
use crate::AppState;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health_check_handler))
        .route("/api/gallery", post(create_gallery))
        .route("/api/gallery/:id", get(get_gallery))
        .route("/api/gallery/:id/upload", post(upload_gallery))
        .route("/api/gallery/:id/download", get(download_gallery))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)) // 50 MB
        .layer(CorsLayer::permissive())
        .fallback_service(ServeDir::new("static"))
        .with_state(state)
}


use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use crate::file_ops::{download_gallery, upload_gallery};
use crate::handler::{create_gallery, get_gallery, health_check_handler};
use crate::AppState;

async fn spa_fallback() -> impl IntoResponse {
    match tokio::fs::read_to_string("static/index.html").await {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "index.html not found").into_response(),
    }
}

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        // API routes
        .route("/api/health", get(health_check_handler))
        .route("/api/gallery", post(create_gallery))
        .route("/api/gallery/:id", get(get_gallery))
        .route("/api/gallery/:id/upload", post(upload_gallery))
        .route("/api/gallery/:id/download", get(download_gallery))

        // serve static assets like /assets/*
        .nest_service("/assets", ServeDir::new("static/assets"))

        // serve index.html for everything else (SPA routing)
        .fallback(get(spa_fallback))

        .layer(DefaultBodyLimit::max(200 * 1024 * 1024))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

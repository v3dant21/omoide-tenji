use crate::handler::{health_check_handler, image_handler};
use axum::routing::post;
use::axum::{
    routing::{get, Router},
    
};
use::tower_http::services::ServeDir;


pub fn create_routes() -> Router {
    let router = Router::new()
        .route("/api/health", get(health_check_handler))
        .route("/api/image", post(image_handler))
        .nest_service("/", ServeDir::new("static"));


    return router;

}
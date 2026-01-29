use axum::{http::{header::{ACCEPT, CONTENT_TYPE, AUTHORIZATION},HeaderValue, Method},Router};
use route::create_routes;
use tower_http::cors:: CorsLayer;

mod route;
mod handler;
#[tokio::main]
async fn main() {

    let cors_config = CorsLayer::new()
        .allow_credentials(true)
        .allow_origin("http://localhost:5000".parse::<HeaderValue>().unwrap())
        
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let app: Router= create_routes().layer(
        cors_config
    );
    println!("server is running at http://localhost:5000");
    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        
        .serve(app.into_make_service())
        .await
        .unwrap();

    


}
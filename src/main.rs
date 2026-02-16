use aws_sdk_s3::Client;

mod file_ops;
mod handler;
mod routes;
mod s3;

#[derive(Clone)]
pub struct AppState {
    pub s3_client: Client,
    pub bucket: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let bucket = std::env::var("S3_BUCKET").expect("S3_BUCKET must be set");
    let region = std::env::var("AWS_REGION").unwrap_or_else(|_| "not set".to_string());


    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let s3_client = Client::new(&config);


    let state = AppState { s3_client, bucket };
    let app = routes::create_routes(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000")
        .await
        .expect("Failed to bind to port 5000");

    println!("Server running at http://localhost:5000");

    axum::serve(listener, app).await.unwrap();
}
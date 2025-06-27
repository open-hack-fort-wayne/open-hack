use axum::{
    routing::{get, post},
    Router,
    Extension,
};
use openhack::{Config, OpenHack};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // Initialize the OpenHack backend
    let openhack = OpenHack::init(
        &Config::builder()
            .db_url("postgresql://localhost:5432/openhack")
            .password_secret("bad-secret")
            .build(),
    )
    .await
    .unwrap();

    // Configure CORS for frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create API routes
    let app = Router::new()
        .route("/api/health", get(health_check))
        // Add your API routes here
        .layer(cors)
        .layer(Extension(openhack));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("🚀 API server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

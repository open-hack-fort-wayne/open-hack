use ::axum::{Router, routing};
use ::openhack::{Config, OpenHack};
use ::tokio::net::TcpListener;

mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ::dotenvy::dotenv().ok();

    let config = &Config::builder()
        .db_url(std::env::var("DATABASE_URL")?)
        .password_secret(std::env::var("PASSWORD_SECRET")?)
        .build();

    let openhack = OpenHack::init(config).await?;
    let keys = crate::api::Keys::new(std::env::var("JWT_SECRET")?.as_bytes());

    let openhack_www = Router::new()
        .route("/.ping", routing::get(async || "PONG"))
        .nest("/api", crate::api::routes(openhack, keys));

    let address = TcpListener::bind(std::env::var("IP_ADDRESS")?).await?;
    ::axum::serve(address, openhack_www).await?;
    Ok(())
}

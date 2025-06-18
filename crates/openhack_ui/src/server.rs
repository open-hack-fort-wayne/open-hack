#![cfg(feature = "server")]

use axum::Extension;
use dioxus::prelude::*;

#[cfg(feature = "server")]
pub async fn launch(component: fn() -> Element) {
    use dioxus::cli_config::{server_ip, server_port};
    use openhack::{Config, OpenHack};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use tokio::net::TcpListener;

    let openhack = OpenHack::init(
        &Config::builder()
            .db_url("postgresql://localhost:5432/openhack")
            .password_secret("bad-secret")
            .build(),
    )
    .await
    .unwrap();

    // Get the address the server should run on. If the CLI
    // is running, the CLI proxies fullstack into the main
    // address and we use the generated address the CLI gives us.
    let ip = server_ip().unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = server_port().unwrap_or(8080);
    let address = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(address).await.unwrap();
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::default(), component)
        .layer(Extension(openhack))
        .into_make_service();
    axum::serve(listener, router).await.unwrap();
}

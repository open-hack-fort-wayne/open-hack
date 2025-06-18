mod backend;
mod gui;
mod server;

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(gui::App);
}

#[cfg(feature = "server")]
fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            server::launch(gui::App).await;
        });
}

#![cfg(feature = "web")]

use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    const FAVICON: Asset = asset!("/assets/favicon.ico");
    const MAIN_CSS: Asset = asset!("/assets/main.css");

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

#[component]
fn Hero() -> Element {
    const HEADER_SVG: Asset = asset!("/assets/header.jpg");
    let mut status = use_signal(|| None::<bool>);

    rsx! {
        div {
            id: "hero",
            h1 { "Fort Wayne Open Hack" }
            img { src: HEADER_SVG, id: "header" }
            div {
                id: "links",
                a { href: "https://discord.gg/cXXpCN99", "💬 Join Discord" }
            }
            button {
                onclick: move |_| {
                    async move {
                        let result = crate::backend::create_user().await;
                        status.set(Some(result.is_ok()));
                    }
                },
                "Sign Up"
            }
            div {
                match *status.read() {
                    None => "",
                    Some(true) => "Good to go",
                    Some(false) => "didn't go so well",
                }
            }
        }
    }
}

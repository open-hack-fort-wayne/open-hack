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

    rsx! {
        div {
            id: "hero",
            h1 { "Fort Wayne Open Hack" }
            img { src: HEADER_SVG, id: "header" }
            div {
                id: "links",
                a { href: "https://discord.gg/cXXpCN99", "💬 Join Discord" }
            }
        }
    }
}

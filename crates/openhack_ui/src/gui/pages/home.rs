use crate::gui::sections::UpcomingEvents;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
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
        UpcomingEvents {  }
    }
}

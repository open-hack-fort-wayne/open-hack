#![cfg(feature = "web")]
#![allow(non_snake_case)]

use dioxus::prelude::*;

mod forms;
mod pages;
use pages::{Home, UserSignIn, UserSignOut, UserSignUp};

#[derive(Clone, Copy)]
struct LoggedIn(pub bool);

#[component]
pub fn App() -> Element {
    const FAVICON: Asset = asset!("/assets/favicon.ico");
    const MAIN_CSS: Asset = asset!("/assets/main.css");
    let logged_in = use_server_future(|| async move {
        crate::backend::get_user_id()
            .await
            .map(|arg| arg.is_some())
            .unwrap()
    })?;

    provide_context(Signal::new(LoggedIn(logged_in.read_unchecked().unwrap())));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

fn logged_in() -> Signal<LoggedIn> {
    use_context::<Signal<LoggedIn>>()
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/sign-up")]
    UserSignUp {},

    #[route("/login")]
    UserSignIn {},

    #[route("/logout")]
    UserSignOut {},
}

#[component]
fn Navbar() -> Element {
    let here = logged_in();

    rsx! {
        div {
            id: "navbar",
            Link { to: Route::Home {}, "🏠 Home" }
            " | "
            {if here.read().0 {
                rsx!{
                    Link { to: Route::UserSignOut {}, "👋 Logout" }
                }
            } else {
                rsx!{
                    Link { to: Route::UserSignUp {}, "✍️ Sign Up" }
                    " | "
                    Link { to: Route::UserSignIn {}, "🔓 Login" }
                }
            }}
        }

        Outlet::<Route> {}
    }
}

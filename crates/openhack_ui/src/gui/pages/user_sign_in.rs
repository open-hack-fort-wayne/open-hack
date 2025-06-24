use crate::gui::forms::SignInForm;
use dioxus::prelude::*;

pub fn UserSignIn() -> Element {
    rsx! {
        div {
            class: "signin-page",
            h1 { "Login" },
            SignInForm {}
        }
    }
}

use super::super::forms::SignUpForm;
use dioxus::prelude::*;

pub fn UserSignUp() -> Element {
    rsx! {
        div {
            class: "signup-page",
            h1 { "Sign Up" }
            SignUpForm {}
        }
    }
}

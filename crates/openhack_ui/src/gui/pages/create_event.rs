use crate::gui::forms::CreateEventForm;
use dioxus::prelude::*;

#[component]
pub fn CreateEvent() -> Element {
    rsx! {
        div {
            h1 { "Create Event" }
            CreateEventForm {}
        }
    }
}

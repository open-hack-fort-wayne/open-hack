use dioxus::prelude::*;

#[component]
pub fn UpcomingEvents() -> Element {
    let events = use_server_future(|| async move {
        crate::backend::upcoming_events().await.unwrap_or_default()
    })?;

    rsx! {
        h2 { "Upcoming Events" }
        ul {
            for event in events.read().clone().unwrap().iter() {
                li {
                    "{event.name} @ {event.scheduled_date}"
                    br {}
                    "{event.details}"
                }
            }
        }
    }
}

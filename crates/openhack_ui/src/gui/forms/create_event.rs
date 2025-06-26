use dioxus::prelude::*;

pub fn CreateEventForm() -> Element {
    let mut title = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut date = use_signal(|| "".to_string());
    let mut location = use_signal(|| "".to_string());
    let mut minutes = use_signal(|| 30u16);
    let mut error_message = use_signal(|| None::<String>);
    let nav = use_navigator();

    let on_submit = {
        move |event: FormEvent| async move {
            event.prevent_default();

            let result = crate::backend::create_event(
                title.read().to_string(),
                description.read().to_string(),
                date.read().to_string(),
                location.read().to_string(),
                minutes.read().to_owned(),
            )
            .await;

            match result {
                Ok(_) => {
                    error_message.set(None);
                    nav.push("/");
                }
                Err(err) => {
                    error_message.set(Some(format!("Event creation failed: {}", err)));
                    return;
                }
            }
        }
    };

    rsx! {
        form { onsubmit: on_submit,
            div {
                label { "Title:" }
                input {
                    r#type: "text",
                    value: "{title}",
                    oninput: move |e| title.set(e.value()),
                    required: true,
                }
            }
            div {
                label { "Description:" }
                textarea {
                    value: "{description}",
                    oninput: move |e| description.set(e.value()),
                    required: true,
                }
            }
            div {
                label { "Date:" }
                input {
                    r#type: "datetime-local",
                    value: "{date}",
                    oninput: move |e| date.set(e.value()),
                    required: true,
                }
            }
            div {
                label { "Duration (minutes):" },
                input {
                    r#type: "number",
                    value: "{minutes}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<u16>() {
                            minutes.set(val);
                        }
                    },
                    min: "1",
                    required: true,
                }
            }
            div {
                label { "Location:" }
                input {
                    r#type: "text",
                    value: "{location}",
                    oninput: move |e| location.set(e.value()),
                    required: true,
                }
            }
            button { r#type: "submit", "Create Event" }
            if let Some(err) = &*error_message.read() {
                p { style: "color: red;", "{err}" }
            }
        }
    }
}

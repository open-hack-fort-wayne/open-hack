use dioxus::prelude::*;

pub fn SignUpForm() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut confirm_password = use_signal(|| "".to_string());
    let mut error_message = use_signal(|| None::<String>);

    let on_submit = {
        move |event: FormEvent| async move {
            event.prevent_default();
            if *password.read() != *confirm_password.read() {
                error_message.set(Some("Passwords do not match".to_string()));
                return;
            };

            let result = crate::backend::signup(
                username.read().to_string(),
                password.read().to_string(),
                email.read().to_string(),
            )
            .await;

            match result {
                Ok(_) => {
                    // Handle successful signup, e.g., redirect or show a success message
                    error_message.set(None);
                }
                Err(err) => {
                    error_message.set(Some(format!("Signup failed: {}", err)));
                    return;
                }
            }
        }
    };

    rsx! {
        form { onsubmit: on_submit,
            div {
                label { "Username:" }
                input {
                    r#type: "text",
                    value: "{username}",
                    oninput: move |e| username.set(e.value()),
                    required: true,
                }
            }
            div {
                label { "Email:" }
                input {
                    r#type: "email",
                    value: "{email}",
                    oninput: move |e| email.set(e.value()),
                    required: true,
                }
            }
            div {
                label { "Password:" }
                input {
                    r#type: "password",
                    value: "{password}",
                    oninput: move |e| password.set(e.value()),
                    required: true,
                }
            }
            div {
                label { "Confirm Password:" }
                input {
                    r#type: "password",
                    value: "{confirm_password}",
                    oninput: move |e| confirm_password.set(e.value()),
                    required: true,
                }
            }
            button { r#type: "submit", "Sign Up" }
            if let Some(error) = &*error_message.read() {
                div { style: "color: red;", "{error}" }
            }
        }
    }
}

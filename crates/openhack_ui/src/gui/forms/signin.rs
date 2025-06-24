use crate::gui::Route;
use dioxus::prelude::*;

pub fn SignInForm() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error_message = use_signal(|| String::new());
    let nav = use_navigator();

    let on_submit = move |event: FormEvent| async move {
        event.prevent_default();

        if email.read().is_empty() || password.read().is_empty() {
            error_message.set("Email and password cannot be empty".to_string());
            return;
        } else {
            error_message.set("".to_string());
        }

        let result =
            crate::backend::login(email.read().to_string(), password.read().to_string()).await;

        match result {
            Ok(_) => {
                crate::gui::logged_in().write().0 = true;
                nav.push(Route::Home {});
            }
            Err(e) => {
                error_message.set(format!("Login failed: {}", e));
            }
        }
    };

    rsx! {
        form {
            onsubmit: on_submit,
            class: "signin-form",
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
            button { "Sign In" }
            if !error_message.read().is_empty() {
                p { class: "error-message", "{error_message}" }
            }
        }
    }
}

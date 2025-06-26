use dioxus::prelude::*;

pub fn UserSignOut() -> Element {
    let nav = use_navigator();

    use_server_future(move || async move {
        let _signout = crate::backend::signout().await;
        crate::gui::logged_in().write().0 = false;
        nav.push(crate::gui::Route::Home {});
    });

    rsx! {
        div {
            class: "signout-page",
            h1 { "Logging Out"}
        }
    }
}

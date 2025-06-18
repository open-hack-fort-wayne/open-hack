#[cfg(feature = "server")]
use axum::Extension;
use dioxus::prelude::*;

#[server(CreateUser)]
pub async fn create_user() -> Result<(), ServerFnError> {
    use openhack::{command::create_user::CreatUser, Context, OpenHack};

    let openhack: Extension<OpenHack> = extract().await?;
    let runner = openhack.runner(&Context::Root);
    let create_user = &CreatUser::builder()
        .password("ineverusegoodpasswords")
        .email("bfalk@rofltown.com")
        .username("bdawg")
        .build();
    runner.run(create_user).await?;
    Ok(())
}

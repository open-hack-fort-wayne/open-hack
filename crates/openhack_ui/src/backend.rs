#[cfg(feature = "server")]
use axum::Extension;
use dioxus::prelude::*;

#[server(SignUp)]
pub async fn signup(
    username: String,
    password: String,
    email: String,
) -> Result<(), ServerFnError> {
    use openhack::{command::create_user::CreatUser, Context, OpenHack};

    let openhack: Extension<OpenHack> = extract().await?;
    let runner = openhack.runner(&Context::Root);
    let create_user = &CreatUser::builder()
        .password(password)
        .email(email)
        .username(username)
        .build();
    runner.run(create_user).await?;
    Ok(())
}

#[server(Login)]
pub async fn login(email: String, password: String) -> Result<(), ServerFnError> {
    use crate::server::authorization::Keys;
    use openhack::{command::login_user::LoginUser, Context, OpenHack};

    let openhack: Extension<OpenHack> = extract().await?;
    let keys: Extension<Keys> = extract().await?;
    let runner = openhack.runner(&Context::Nobody);
    let login = &LoginUser::builder().password(password).email(email).build();
    let user = runner.run(login).await?;
    let jwt = keys
        .encode_jwt(user.id.0)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let ctx = server_context();
    ctx.response_parts_mut().headers.insert(
        axum::http::header::SET_COOKIE,
        axum::http::HeaderValue::from_str(&format!(
            "jwt={}; Path=/; HttpOnly; SameSite=Strict",
            jwt
        ))?,
    );
    Ok(())
}

#[server(GetUserId)]
pub async fn get_user_id() -> Result<Option<i32>, ServerFnError> {
    use crate::server::authorization::Claims;

    Ok(server_context()
        .request_parts()
        .extensions
        .get::<Claims>()
        .map(|claims| claims.sub))
}

#[server(SignOut)]
pub async fn signout() -> Result<(), ServerFnError> {
    let ctx = server_context();
    ctx.response_parts_mut().headers.insert(
        axum::http::header::SET_COOKIE,
        axum::http::HeaderValue::from_str(&format!(
            "jwt=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0"
        ))?,
    );
    Ok(())
}

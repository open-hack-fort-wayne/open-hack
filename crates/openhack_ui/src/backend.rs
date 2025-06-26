use crate::structs::Event;
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

#[server(CreateEvent)]
pub async fn create_event(
    title: String,
    description: String,
    date: String,
    location: String,
    minutes: u16,
) -> Result<(), ServerFnError> {
    use chrono::Duration;
    use openhack::common::DateTimeUtc;
    use openhack::entity::UserId;
    use openhack::{command::create_event::CreateEvent, Context, OpenHack};

    let user_id = get_user_id()
        .await?
        .ok_or_else(|| ServerFnError::new("User not logged in".to_string()))?;
    let user_id = UserId(user_id);

    let date = chrono::NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M")
        .map_err(|e| ServerFnError::new(format!("Error parsing date: {}", e)))
        .map(|naive| DateTimeUtc::from_naive_utc_and_offset(naive, chrono::Utc))?
        .with_timezone(&chrono::Utc);

    let openhack: Extension<OpenHack> = extract().await?;
    let context = &Context::User(user_id);
    let runner = openhack.runner(context);
    let create_event = &CreateEvent::builder()
        .name(title)
        .location(location)
        .details(description)
        .scheduled_date(date)
        .duration(Duration::minutes(minutes.into()))
        .build();
    runner.run(create_event).await?;
    Ok(())
}

#[server(UpcomingEvents)]
pub async fn upcoming_events() -> Result<Vec<Event>, ServerFnError> {
    use openhack::{report::search_events::SearchEvents, Context, OpenHack};
    let openhack: Extension<OpenHack> = extract().await?;
    let report = openhack.reporter(&Context::Nobody);
    let list_events = &SearchEvents::builder().build();
    let events = report.run(list_events).await?;
    Ok(events.data.into_iter().map(Into::into).collect())
}

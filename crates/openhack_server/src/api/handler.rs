use crate::api::{AppContext, Response, ResponseWithHeaders};
use ::axum::{
    Extension, Json,
    extract::{Query, State},
};
use ::openhack::{OpenHack, command::CommandExt, report::ReportExt};
use ::serde::Serialize;

/// # Mount Command
///
/// This is a generic [CommandExt] handler that
/// will run the provided command and return it's
/// success type encoded as JSON with a status code
/// of [::axum::http::StatusCode::OK] or Json hash
/// object with the following format:
///
/// ```json
/// {"status":"error", "message":"some formatted string"}
/// ```
///
/// and a status of [::axum::http::StatusCode::BAD_REQUEST]
pub async fn command<T>(
    State(state): State<OpenHack>,
    AppContext(context): AppContext,
    Json(cmd): Json<T>,
) -> Response<T::Success>
where
    T: CommandExt,
    T::Success: Serialize,
    T::Failure: ToString,
{
    let runner = state.runner(&context);
    let result = runner.run(&cmd).await;
    Response::from_result(result)
}

/// # Mount Report
///
/// This is a generic [ReportExt] handler that
/// will run the provided report and return it's
/// success type encoded as JSON with a status code
/// of [::axum::http::StatusCode::OK] or Json hash
/// object with the following format:
///
/// ```json
/// {"status":"error", "message":"some formatted string"}
/// ```
///
/// and a status of [::axum::http::StatusCode::BAD_REQUEST]
pub async fn report<T>(
    State(state): State<OpenHack>,
    AppContext(context): AppContext,
    Query(report): Query<T>,
) -> Response<T::Success>
where
    T: ReportExt,
    T::Success: Serialize,
    T::Failure: ToString,
{
    let reporter = state.reporter(&context);
    let result = reporter.run(&report).await;
    Response::from_result(result)
}

pub async fn sign_in(
    State(state): State<OpenHack>,
    AppContext(context): AppContext,
    Extension(keys): Extension<crate::api::Keys>,
    Json(cmd): Json<::openhack::command::login_user::LoginUser>,
) -> ResponseWithHeaders<::openhack::entity::User> {
    let runner = state.runner(&context);
    let result = runner.run(&cmd).await;
    let jwt = result
        .as_ref()
        .ok()
        .and_then(|user| keys.encode(user.id).ok());

    let mut headers = axum::http::HeaderMap::new();
    if let Some(jwt) = jwt {
        let cookie = format!("jwt={jwt}; Path=/; HttpOnly; SameSite=Strict");
        headers.insert(
            axum::http::header::SET_COOKIE,
            axum::http::HeaderValue::from_str(&cookie).unwrap(),
        );
    }
    Response::from_result(result).with_headers(headers)
}

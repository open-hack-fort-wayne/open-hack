use super::Keys;
use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware, Extension, RequestExt, Router,
};
use axum_extra::extract::CookieJar;

pub async fn jwt_middleware(
    mut req: Request<Body>,
    next: middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let headers = req.headers();
    let jar = CookieJar::from_headers(headers);
    let jwt = jar.get("jwt").map(|cookie| cookie.value());

    match jwt {
        Some(token) => {
            let keys = req
                .extensions()
                .get::<Keys>()
                .ok_or(StatusCode::UNAUTHORIZED)?;
            match keys.decode_claims(token) {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);
                    Ok(next.run(req).await)
                }
                Err(_) => Ok(next.run(req).await),
            }
        }
        _ => Ok(next.run(req).await),
    }
}

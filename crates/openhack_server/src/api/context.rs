use crate::api::{Keys, Response};
use ::axum::extract::FromRequestParts;
use ::axum_extra::extract::CookieJar;
use ::openhack::{Context, OpenHack};

pub struct AppContext(pub Context);

impl FromRequestParts<OpenHack> for AppContext {
    type Rejection = Response<()>;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &OpenHack,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let keys = parts
            .extensions
            .get::<Keys>()
            .ok_or_else(|| Response::failure("missing jwt keys"))?;

        match jar.get("jwt").map(|cookie| cookie.value()) {
            None => Ok(Self(Context::Nobody)),
            Some(jwt) => match keys.decode(jwt) {
                Ok(claims) => Ok(Self(Context::User(claims.sub.into()))),
                Err(_) => Err(Response::failure("error decoding claims")),
            },
        }
    }
}

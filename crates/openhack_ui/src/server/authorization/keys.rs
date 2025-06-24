use super::Claims;
use anyhow::{Context as _, Error, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use std::sync::Arc;

#[derive(Clone)]
pub struct Keys {
    inner: Arc<Inner>,
}

struct Inner {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            inner: Arc::new(Inner {
                encoding: EncodingKey::from_secret(secret),
                decoding: DecodingKey::from_secret(secret),
            }),
        }
    }

    pub fn encode_jwt(&self, user_id: i32) -> Result<String> {
        let now = Utc::now();
        let expiration = now
            .checked_add_signed(Duration::hours(24))
            .context("expected an expiration in the future")?
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: expiration,
            iat: now.timestamp() as usize,
        };

        jsonwebtoken::encode(&Header::default(), &claims, &self.inner.encoding)
            .context("unable to build jwt")
    }

    pub fn decode_claims(&self, jwt: &str) -> Result<Claims> {
        jsonwebtoken::decode(jwt, &self.inner.decoding, &Validation::default())
            .context("unable to decode jwt")
            .map(|data| data.claims)
    }
}

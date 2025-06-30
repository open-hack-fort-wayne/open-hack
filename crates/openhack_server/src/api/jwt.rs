use ::anyhow::{Context as _, Result};
use ::chrono::{Duration, Utc};
use ::jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use ::openhack::entity::UserId;
use ::serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Keys {
    inner: Arc<InnerKeys>,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        let inner = Arc::new(InnerKeys {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        });
        Self { inner }
    }

    pub fn encode(&self, user_id: UserId) -> Result<String> {
        let now = Utc::now();
        let expiration = now
            .checked_add_signed(Duration::hours(24))
            .context("expected an expiration in the future")?
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.0,
            exp: expiration,
            iat: now.timestamp() as usize,
        };

        jsonwebtoken::encode(&Header::default(), &claims, &self.inner.encoding)
            .context("unable to build jwt")
    }

    pub fn decode(&self, jwt: &str) -> Result<Claims> {
        jsonwebtoken::decode(jwt, &self.inner.decoding, &Validation::default())
            .context("unable to decode jwt")
            .map(|data| data.claims)
    }
}

struct InnerKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    pub iat: usize,
}

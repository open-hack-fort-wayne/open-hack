use crate::{command::CommandRunner, config::Config, context::Context, env::Environment};
use ::anyhow::{Context as _, Result};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OpenHack {
    env: Arc<Environment>,
}

impl OpenHack {
    pub async fn init(config: &Config) -> Result<Self> {
        let pool = sqlx::PgPool::connect(&config.db_url).await?;

        let hasher = openhack_auth::PasswordHasher::new(config.password_secret.as_bytes())
            .ok()
            .context("couldn't create password hasher")?;

        let env = Environment::builder()
            .database(pool)
            .password_hasher(hasher)
            .build();

        Ok(Self { env: Arc::new(env) })
    }

    pub fn runner<'a>(&'a self, context: &'a Context) -> CommandRunner<'a, Environment> {
        CommandRunner::new(context, self.env.as_ref())
    }
}

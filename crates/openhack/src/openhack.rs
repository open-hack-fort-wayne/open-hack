use crate::{
    command::CommandRunner, config::Config, context::Context, env::Environment,
    report::ReportRunner,
};
use ::anyhow::{Context as _, Result};
use ::openhack_auth::PasswordHasher;
use std::sync::Arc;

/// # OpenHack
///
/// Singleton which provides access via [OpenHack::runner].  This
/// runner executes provided commands found in the submodules under
/// [crate::command].
///
#[derive(Debug, Clone)]
pub struct OpenHack {
    env: Arc<Environment>,
}

impl OpenHack {
    pub async fn init(config: &Config) -> Result<Self> {
        let pool = sqlx::PgPool::connect(&config.db_url).await?;

        let hasher = PasswordHasher::new(config.password_secret.as_bytes())
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

    pub fn reporter<'a>(&'a self, context: &'a Context) -> ReportRunner<'a, Environment> {
        ReportRunner::new(context, self.env.as_ref())
    }
}

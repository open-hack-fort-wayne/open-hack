use crate::{context::Context, env::Env};

/// # Command Trait
///
/// This is a simple strategy pattern trait designed to isolate
/// calls into the system.  The [CommandRunner] will provided the
/// necessary [Context] and [Env].
///
pub trait ReportExt {
    type Success;
    type Failure;

    fn exec(
        &self,
        ctx: &Context,
        env: &impl Env,
    ) -> impl Future<Output = Result<Self::Success, Self::Failure>>;
}

pub struct ReportRunner<'a, E: Env> {
    ctx: &'a Context,
    env: &'a E,
}

impl<'a, E: Env> ReportRunner<'a, E> {
    #[doc(hidden)]
    pub(crate) fn new(ctx: &'a Context, env: &'a E) -> Self {
        Self { ctx, env }
    }

    /// Executes [CommandExt] and returns it's result.
    pub async fn run<CMD>(&self, command: &CMD) -> Result<CMD::Success, CMD::Failure>
    where
        CMD: ReportExt,
    {
        command.exec(self.ctx, self.env).await
    }
}

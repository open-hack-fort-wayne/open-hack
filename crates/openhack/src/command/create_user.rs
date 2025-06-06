use super::core::CommandExt;
use crate::entity::{Email, Password, User};
use crate::env::query::{InsertUser, InsertUserError};
use ::validator::Validate;

#[derive(derive_more::Debug, Clone, Validate, bon::Builder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatUser {
    #[builder(into)]
    pub username: String,

    #[builder(into)]
    #[validate(nested)]
    pub password: Password,

    #[builder(into)]
    #[validate(nested)]
    pub email: Email,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateUserError {
    #[error(transparent)]
    Validations(#[from] validator::ValidationErrors),

    #[error("duplicate username or email")]
    DuplicateEmailOrUsername,

    #[error(transparent)]
    Database(sqlx::Error),

    #[error("bad password: {0:?}")]
    BadPassword(openhack_auth::HashError),
}

impl CommandExt for CreatUser {
    type Success = User;
    type Failure = CreateUserError;

    async fn exec(
        &self,
        _: &crate::context::Context,
        env: &impl crate::env::Env,
    ) -> Result<Self::Success, Self::Failure> {
        self.validate()?;

        let password_hash = env
            .hash_password(self.password.as_str())
            .map_err(CreateUserError::BadPassword)?;

        env.exec_query(&InsertUser {
            username: self.username.clone(),
            email: self.email.clone(),
            password_hash: password_hash.into(),
        })
        .await
        .map_err(CreateUserError::from)
    }
}

mod impls {
    use super::*;
    impl From<InsertUserError> for CreateUserError {
        fn from(value: InsertUserError) -> Self {
            match value {
                InsertUserError::DuplicateEmailOrUsername => Self::DuplicateEmailOrUsername,
                InsertUserError::Database(err) => Self::Database(err),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        context::Context,
        entity::UserId,
        support::{env::*, prelude::*},
    };
    use ::mockall::predicate;
    type Result<T> = std::result::Result<T, CreateUserError>;

    #[rstest]
    #[tokio::test]
    async fn happy_path(mut mock_env: MockEnvironment) -> Result<()> {
        mock_env
            .expect_hash_password()
            .with(predicate::eq("superbadpassword"))
            .returning(|_| Ok(String::from("super-hash")));

        mock_env
            .expect_exec_query()
            .withf(|insert: &InsertUser| {
                insert.username == "jdong"
                    && insert.password_hash.as_str() == "super-hash"
                    && insert.email.as_str() == "jdong@hotmail.com"
            })
            .returning(|_| Ok(User::default()));

        let ctx = Context::default();

        let create_user = CreatUser::builder()
            .password("superbadpassword")
            .email("jdong@hotmail.com")
            .username("jdong")
            .build();

        let user = create_user.exec(&ctx, &mock_env).await?;
        assert_eq!(user.id, UserId(0));

        Ok(())
    }
}

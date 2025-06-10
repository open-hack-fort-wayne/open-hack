use super::core::CommandExt;
use crate::entity::{EmailAddress, Password, User, UserId};
use crate::env::query::{FetchPasswordHash, FetchPasswordHashError, FetchUserForLogin};
use ::validator::Validate;

#[derive(derive_more::Debug, Clone, Validate, bon::Builder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LoginUser {
    #[builder(into)]
    #[validate(nested)]
    pub password: Password,

    #[builder(into)]
    #[validate(nested)]
    pub email: EmailAddress,
}

#[derive(Debug, thiserror::Error)]
pub enum LoginUserError {
    #[error("missing {0:?}")]
    MissingUser(UserId),

    #[error("missing hash")]
    MissingHash,

    #[error(transparent)]
    Database(sqlx::Error),

    #[error("{0:?}")]
    BadPassword(openhack_auth::HashError),
}

impl CommandExt for LoginUser {
    type Success = User;
    type Failure = LoginUserError;

    async fn exec(
        &self,
        _: &crate::context::Context,
        env: &impl crate::env::Env,
    ) -> Result<Self::Success, Self::Failure> {
        let user_hash = env
            .exec_query(&FetchPasswordHash {
                email: self.email.clone(),
            })
            .await?
            .ok_or_else(|| LoginUserError::MissingHash)?;

        env.verify_password(self.password.as_str(), user_hash.hash.as_str())
            .map_err(LoginUserError::BadPassword)?;

        env.exec_query(&FetchUserForLogin(user_hash.user_id))
            .await
            .map_err(LoginUserError::Database)?
            .ok_or_else(|| LoginUserError::MissingUser(user_hash.user_id))
    }
}

mod impls {
    use super::*;

    impl From<FetchPasswordHashError> for LoginUserError {
        fn from(value: FetchPasswordHashError) -> Self {
            match value {
                FetchPasswordHashError::Database(db_err) => Self::Database(db_err),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::entity::UserHash;
    use crate::support::{env::*, prelude::*};
    type Result<T> = std::result::Result<T, LoginUserError>;

    #[fixture]
    fn login() -> LoginUser {
        LoginUser::builder()
            .email("jdong@hotmail.com")
            .password("jdong-baby")
            .build()
    }

    fn jdongs_user_hash() -> UserHash {
        UserHash::builder().user_id(42).hash("rofl-hash").build()
    }

    #[rstest]
    #[tokio::test]
    async fn happy_path(mut mock_env: MockEnvironment, login: LoginUser) -> Result<()> {
        mock_env
            .expect_exec_query()
            .withf(|x: &FetchPasswordHash| x.email.as_str() == "jdong@hotmail.com")
            .returning(|_| Ok(Some(jdongs_user_hash())));

        mock_env
            .expect_verify_password()
            .withf(|password, hash| {
                assert_eq!(password, "jdong-baby");
                assert_eq!(hash, "rofl-hash");
                true
            })
            .returning(|_, _| Ok(()));

        mock_env
            .expect_exec_query()
            .withf(|x: &FetchUserForLogin| x.0 == UserId(42))
            .returning(|_| {
                Ok(Some(User {
                    id: UserId(42),
                    ..Default::default()
                }))
            });

        let ctx = Context::default();
        let user = login.exec(&ctx, &mock_env).await?;
        assert_eq!(user.id, UserId(42));

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn missing_hash(mut mock_env: MockEnvironment, login: LoginUser) -> Result<()> {
        mock_env
            .expect_exec_query()
            .withf(|x: &FetchPasswordHash| x.email.as_str() == "jdong@hotmail.com")
            .returning(|_| Ok(None));

        let ctx = Context::default();
        let err = login.exec(&ctx, &mock_env).await.expect_err("missing hash");
        assert!(matches!(err, LoginUserError::MissingHash));
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn bad_password(mut mock_env: MockEnvironment, login: LoginUser) -> Result<()> {
        mock_env
            .expect_exec_query()
            .withf(|x: &FetchPasswordHash| x.email.as_str() == "jdong@hotmail.com")
            .returning(|_| Ok(Some(jdongs_user_hash())));

        mock_env
            .expect_verify_password()
            .withf(|password, hash| {
                assert_eq!(password, "jdong-baby");
                assert_eq!(hash, "rofl-hash");
                true
            })
            .returning(|_, _| Err(openhack_auth::HashError::Password));

        let ctx = Context::default();
        let err = login.exec(&ctx, &mock_env).await.expect_err("bad password");
        assert!(matches!(err, LoginUserError::BadPassword(_)));
        Ok(())
    }
}

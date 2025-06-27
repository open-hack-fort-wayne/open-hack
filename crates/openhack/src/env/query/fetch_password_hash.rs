use super::*;
use crate::entity::{EmailAddress, UserHash};

#[derive(DebugMore, Clone, Serialize, Deserialize, bon::Builder)]
pub struct FetchPasswordHash {
    #[builder(into)]
    pub email: EmailAddress,
}

#[derive(Debug, thiserror::Error)]
pub enum FetchPasswordHashError {
    #[error(transparent)]
    Database(sqlx::Error),
}

impl Query for FetchPasswordHash {
    type Success = Option<UserHash>;
    type Failure = FetchPasswordHashError;

    async fn exec(&self, conn: &PgPool) -> Result<Self::Success, Self::Failure> {
        let user_hash = sqlx::query_as!(
            UserHash,
            r#"
                SELECT
                    password_hash as hash,
                    id as user_id
                FROM users
                WHERE email = LOWER($1)
            "#,
            self.email.as_str()
        )
        .fetch_optional(conn)
        .await
        .map_err(FetchPasswordHashError::Database)?;

        Ok(user_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::UserId;

    #[sqlx::test(fixtures("./fixtures/john-smith.sql"))]
    async fn finds_exact_case_email(pool: PgPool) -> Result<(), FetchPasswordHashError> {
        let ph = FetchPasswordHash::builder()
            .email("john-smith@hotmail.com")
            .build()
            .exec(&pool)
            .await?
            .unwrap();

        assert_eq!(ph.user_id, UserId(1));
        Ok(())
    }

    #[sqlx::test(fixtures("./fixtures/john-smith.sql"))]
    async fn finds_mixed_case_email(pool: PgPool) -> Result<(), FetchPasswordHashError> {
        let ph = FetchPasswordHash::builder()
            .email("John-Smith@HOTMAIL.COM")
            .build()
            .exec(&pool)
            .await?
            .unwrap();

        assert_eq!(ph.user_id, UserId(1));
        Ok(())
    }
}

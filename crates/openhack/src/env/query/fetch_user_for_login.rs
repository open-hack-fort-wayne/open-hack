use super::*;
use crate::entity::{User, UserId};

#[derive(DebugMore, Clone, Serialize, Deserialize)]
pub struct FetchUserForLogin(pub UserId);

impl Query for FetchUserForLogin {
    type Success = Option<User>;
    type Failure = sqlx::Error;

    async fn exec(&self, conn: &sqlx::PgPool) -> Result<Self::Success, Self::Failure> {
        sqlx::query_as!(
            User,
            r#"
                UPDATE users
                SET last_login = NOW()
                WHERE id = $1
                RETURNING
                    id,
                    username,
                    email,
                    created_at as "created_at!",
                    updated_at as "updated_at!",
                    last_login
            "#,
            self.0.0 as i32
        )
        .fetch_optional(conn)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(fixtures("./fixtures/john-smith.sql"))]
    async fn finds_and_updates_last_login(pool: PgPool) -> Result<(), sqlx::Error> {
        let login = FetchUserForLogin(UserId(1));
        let user = login.exec(&pool).await?.unwrap();
        assert_eq!(user.username, "john-smith");
        assert!(user.last_login.is_some());
        Ok(())
    }

    #[sqlx::test]
    async fn missing_user_is_none(pool: PgPool) -> Result<(), sqlx::Error> {
        let login = FetchUserForLogin(UserId(1));
        assert!(login.exec(&pool).await?.is_none());
        Ok(())
    }
}

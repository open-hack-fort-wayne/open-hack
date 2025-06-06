use super::*;
use crate::entity::{User, UserId};

#[derive(DebugMore, Clone, Serialize, Deserialize)]
pub struct FetchUserById(pub UserId);

impl Query for FetchUserById {
    type Success = Option<User>;
    type Failure = sqlx::Error;

    async fn exec(&self, conn: &sqlx::PgPool) -> Result<Self::Success, Self::Failure> {
        sqlx::query_as!(
            User,
            r#"
                SELECT
                    id,
                    username,
                    email,
                    created_at as "created_at!",
                    updated_at as "updated_at!",
                    last_login
                FROM users
                WHERE
                    id = $1
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
    async fn finds_user_by_id(pool: PgPool) -> Result<(), sqlx::Error> {
        let fetch_user = FetchUserById(UserId(1));
        let user = fetch_user.exec(&pool).await?.unwrap();
        assert_eq!(user.username, "john-smith");
        assert!(user.last_login.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn missing_user_is_none(pool: PgPool) -> Result<(), sqlx::Error> {
        let fetch_user = FetchUserById(UserId(1));
        assert!(fetch_user.exec(&pool).await?.is_none());
        Ok(())
    }
}

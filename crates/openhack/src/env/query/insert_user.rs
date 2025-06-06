use super::*;
use crate::entity::{Email, PasswordHash, User};
use ::sqlx::postgres::PgDatabaseError;

#[derive(DebugMore, Clone, Serialize, Deserialize, bon::Builder)]
pub struct InsertUser {
    #[builder(into)]
    pub username: String,

    #[builder(into)]
    pub password_hash: PasswordHash,

    #[builder(into)]
    pub email: Email,
}

#[derive(Debug, thiserror::Error)]
pub enum InsertUserError {
    #[error("email or username already registered")]
    DuplicateEmailOrUsername,

    #[error(transparent)]
    Database(sqlx::Error),
}

impl Query for InsertUser {
    type Success = User;
    type Failure = InsertUserError;

    async fn exec(&self, conn: &PgPool) -> Result<User, InsertUserError> {
        let query = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users ( username, email, password_hash )
                VALUES ( $1, $2, $3 )
                RETURNING
                    id,
                    username,
                    email,
                    created_at as "created_at!",
                    updated_at as "updated_at!",
                    last_login;
            "#,
            &self.username,
            self.email.as_str(),
            self.password_hash.as_str(),
        );

        query.fetch_one(conn).await.map_err(|error| match error {
            sqlx::Error::Database(ref e) => {
                let pg_err: &PgDatabaseError = e.downcast_ref();
                if pg_err.code() == "23505" {
                    InsertUserError::DuplicateEmailOrUsername
                } else {
                    InsertUserError::Database(error)
                }
            }
            remaining => InsertUserError::Database(remaining),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entity::UserId;

    fn create_john_smith() -> InsertUser {
        InsertUser::builder()
            .username("john-smith")
            .password_hash("bad-password")
            .email("john-smith@hotmail.com")
            .build()
    }

    #[sqlx::test]
    async fn creation_works(pool: PgPool) -> Result<(), InsertUserError> {
        let user = create_john_smith().exec(&pool).await?;
        assert!(user.id > UserId(0));
        assert_eq!(&user.username, "john-smith");
        assert_eq!(&user.email, "john-smith@hotmail.com");
        assert!(user.last_login.is_none());
        Ok(())
    }

    #[sqlx::test(fixtures("./fixtures/john-smith.sql"))]
    async fn catches_duplicate(pool: PgPool) -> Result<(), InsertUserError> {
        let dupe = create_john_smith()
            .exec(&pool)
            .await
            .expect_err("expected duplicate error");
        assert!(matches!(dupe, InsertUserError::DuplicateEmailOrUsername));
        Ok(())
    }
}

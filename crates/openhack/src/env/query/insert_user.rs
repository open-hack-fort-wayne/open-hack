use super::*;
use crate::entity::User;
use ::sqlx::postgres::PgDatabaseError;

#[derive(DebugMore, Clone, Serialize, Deserialize)]
pub struct InsertUser {
    pub username: String,

    #[debug(skip)]
    pub password_hash: String,

    #[debug(skip)]
    pub email: String,
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
            &self.email,
            &self.password_hash,
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

    fn create_john_smith() -> InsertUser {
        InsertUser {
            username: "john-smith".to_owned(),
            password_hash: "bad-password".to_owned(),
            email: "john-smith@hotmail.com".to_owned(),
        }
    }

    #[sqlx::test]
    async fn creation_works(pool: PgPool) -> Result<(), InsertUserError> {
        let user = create_john_smith().exec(&pool).await?;
        assert!(user.id > 0);
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

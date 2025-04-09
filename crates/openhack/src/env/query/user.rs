use super::*;
use crate::entity::User;

#[derive(DebugMore, Clone, Serialize, Deserialize)]
pub struct Create {
    pub username: String,

    #[debug(skip)]
    pub password_hash: String,

    #[debug(skip)]
    pub email: String,
}

impl Query<PgPool> for Create {
    type Output = User;

    async fn exec(&self, conn: &PgPool) -> Result<Self::Output> {
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

        Ok(query.fetch_one(conn).await?)
    }
}

use super::{PasswordHash, UserId};

#[derive(Debug, Clone, bon::Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "runtime", derive(sqlx::FromRow))]
pub struct UserHash {
    #[builder(into)]
    pub user_id: UserId,

    #[builder(into)]
    pub hash: PasswordHash,
}

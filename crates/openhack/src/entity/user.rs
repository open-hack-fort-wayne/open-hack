use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "runtime", derive(sqlx::FromRow))]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub last_login: Option<DateTimeUtc>,
}

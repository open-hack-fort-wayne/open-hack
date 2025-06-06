use super::UserId;
use crate::common::DateTimeUtc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, bon::Builder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "runtime", derive(sqlx::FromRow))]
pub struct User {
    #[builder(into)]
    pub id: UserId,

    #[builder(into)]
    pub username: String,

    #[builder(into)]
    pub email: String,

    #[builder(into)]
    pub created_at: DateTimeUtc,

    #[builder(into)]
    pub updated_at: DateTimeUtc,

    #[builder(into)]
    pub last_login: Option<DateTimeUtc>,
}

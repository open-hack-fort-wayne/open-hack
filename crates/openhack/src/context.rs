use crate::entity::UserId;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Context {
    /// Absolute authority
    Root,

    /// On behalf of a user
    User(UserId),

    /// Uknown / anonymous
    #[default]
    Nobody,
}

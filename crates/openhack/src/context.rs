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

impl Context {
    /// # User Id
    ///
    /// Fetches the [UserId] for the context if
    /// one is available.
    ///
    pub fn user_id(&self) -> Option<UserId> {
        match self {
            Self::User(id) => Some(*id),
            Self::Nobody | Self::Root => None,
        }
    }
}

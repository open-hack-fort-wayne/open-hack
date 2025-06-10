#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct EventId(pub i32);

mod impls {
    use super::*;

    impl From<i32> for EventId {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct UserId(pub i64);

mod impls {
    use super::*;

    impl From<i32> for UserId {
        fn from(value: i32) -> Self {
            Self(value as i64)
        }
    }
}

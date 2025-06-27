use crate::common::DateTimeUtc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum DateSelection {
    Before(DateTimeUtc),
    After(DateTimeUtc),
    Between(DateTimeUtc, DateTimeUtc),
}

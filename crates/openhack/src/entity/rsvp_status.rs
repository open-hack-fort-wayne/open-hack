#[derive(Debug, Clone, Copy, PartialEq, Hash, Default, Eq, sqlx::Type)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[sqlx(type_name = "rsvp_status", rename_all = "lowercase")]
pub enum RsvpStatus {
    Yes,
    No,
    #[default]
    Maybe,
}

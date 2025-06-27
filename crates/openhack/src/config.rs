#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, bon::Builder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Config {
    #[builder(into)]
    pub db_url: String,

    #[builder(into)]
    pub password_secret: String,
}

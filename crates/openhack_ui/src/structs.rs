#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub id: i32,
    pub scheduled_date: String,
    pub duration_in_mins: i32,
    pub name: String,
    pub location: String,
    pub details: String,
}

#[cfg(feature = "server")]
impl From<openhack::entity::Event> for Event {
    fn from(value: openhack::entity::Event) -> Self {
        Self {
            id: value.id.0,
            scheduled_date: value.scheduled_date.format("%Y-%m-%d %H:%M").to_string(),
            duration_in_mins: value.duration_in_mins,
            name: value.name,
            location: value.location,
            details: value.details,
        }
    }
}

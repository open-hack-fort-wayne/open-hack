use crate::common::{DateTimeUtc, Duration};
use chrono::Utc;

pub fn today() -> DateTimeUtc {
    Utc::now()
}

pub fn two_days_from_now() -> DateTimeUtc {
    today() + Duration::days(2)
}

pub fn yesterday() -> DateTimeUtc {
    today() - Duration::days(1)
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DateTime(String);

impl TryFrom<DateTime> for chrono::DateTime<chrono::Utc> {
    type Error = chrono::ParseError;

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        chrono::DateTime::parse_from_rfc3339(value.0.as_str()).map(Into::into)
    }
}

//! The `DateTime` type represents a date-time as used in the VCO API.
//!
//! According to the VCO API docs, date-times can be sent to VCO in one of two formats: an RFC3339
//! string or a unix epoch timestamp integer.
//!

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;
use time::format_description::well_known::Rfc3339;
use time::{OffsetDateTime, UtcOffset};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DateTime {
    None,
    Never,
    Stamp(OffsetDateTime),
}

/// The display format of `DateTime` is RFC3339.
impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DateTime::None => write!(f, "{}", "null"),
            DateTime::Never => write!(f, "{}", "never"),
            DateTime::Stamp(inner) => {
                write!(f, "{}", DateTime::offset_date_time_to_rfc3339_string(inner))
            }
        }
    }
}

/// Default strings are RFC3339 strings.
impl TryFrom<&str> for DateTime {
    type Error = DateTimeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_rfc3339(value)
    }
}

/// Default integers are Unix timestamps.
impl TryFrom<i64> for DateTime {
    type Error = DateTimeError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::from_unix_timestamp(value)
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (DateTime::Stamp(a), DateTime::Stamp(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl DateTime {
    /// Create a `DateTime` from an RFC3339-formatted string, converting the result to UTC timezone
    /// as expected by VCO.
    /// TODO is convert to UTC necessary?
    pub fn from_rfc3339(value: &str) -> Result<Self, DateTimeError> {
        Ok(DateTime::Stamp(
            OffsetDateTime::parse(value, &Rfc3339)
                .map_err(|e| DateTimeError::BadDateTimeString(e.to_string()))?
                .to_offset(UtcOffset::UTC),
        ))
    }

    /// Create a `DateTime` from a Unix timestamp value, converting the result to UTC timezone as
    /// expected by VCO.
    /// TODO is convert to UTC necessary?
    pub fn from_unix_timestamp(value: i64) -> Result<Self, DateTimeError> {
        Ok(DateTime::Stamp(
            OffsetDateTime::from_unix_timestamp(value)
                .map_err(|_| DateTimeError::BadUnixTimestamp(value))?
                .to_offset(UtcOffset::UTC),
        ))
    }

    /// Output as an RFC3339-formatted `String`.
    pub fn to_rfc3339(&self) -> Result<String, DateTimeError> {
        match self {
            DateTime::None => Err(DateTimeError::NoRfc3339Equivalent),
            DateTime::Never => Err(DateTimeError::NoRfc3339Equivalent),
            DateTime::Stamp(inner) => Ok(Self::offset_date_time_to_rfc3339_string(&inner)),
        }
    }

    // TODO implement from_ymdhms_utc

    #[inline]
    fn offset_date_time_to_rfc3339_string(inner: &OffsetDateTime) -> String {
        inner
            .format(&Rfc3339)
            .unwrap_or_else(|_| panic!("Internal error: could not convert {inner} to RFC3339"))
    }
}

//
// SERDE
//

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&match self {
            DateTime::None => "null".to_string(),
            DateTime::Never => "0000-00-00 00:00:00".to_string(),
            DateTime::Stamp(inner) => DateTime::offset_date_time_to_rfc3339_string(inner),
        })
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateTimeVisitor;

        impl<'de> serde::de::Visitor<'de> for DateTimeVisitor {
            type Value = DateTime;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("either an RFC3339 date string or an epoch timestamp")
            }

            /// Parse epoch timestamps.
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                DateTime::from_unix_timestamp(v as i64).map_err(|e| E::custom(e.to_string()))
            }

            /// Parse RFC3339 date strings.
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "null" => Ok(DateTime::None),
                    "0000-00-00 00:00:00" => Ok(DateTime::Never),
                    _ => DateTime::from_rfc3339(v).map_err(|e| E::custom(e.to_string())),
                }
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DateTime::None)
            }
        }

        deserializer.deserialize_any(DateTimeVisitor)
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DateTimeError {
    #[error("Bad date/time string format: \"{0}\"")]
    BadDateTimeString(String),

    #[error("Bad unix timestamp: \"{0}\"")]
    BadUnixTimestamp(i64),

    #[error("Cannot convert to RFC3339")]
    NoRfc3339Equivalent,

    #[error("Invalid year: \"{0}\"")]
    InvalidYear(u16),

    #[error("Invalid month: \"{0}\"")]
    InvalidMonth(u8),

    #[error("Invalid day: \"{0}\"")]
    InvalidDay(u8),

    #[error("Invalid hour: \"{0}\"")]
    InvalidHour(u8),

    #[error("Invalid minute: \"{0}\"")]
    InvalidMinute(u8),

    #[error("Invalid second: \"{0}\"")]
    InvalidSecond(u8),
}

//
// TESTS
//

#[cfg(test)]
mod date_time_tests {
    use super::DateTime;
    use serde_json::{self, json};

    /// Test serializing a UTC datetime.
    #[test]
    fn test_datetime_ser() {
        let dt = DateTime::from_rfc3339("2023-01-02T03:04:05.000Z").unwrap();
        let ser = serde_json::ser::to_string(&dt);
        assert!(ser.is_ok());
        assert_eq!(ser.unwrap(), "\"2023-01-02T03:04:05Z\"");
    }

    /// Test serializing a datetime with a non-UTC timezone.
    #[test]
    fn test_datetime_ser_with_tz() {
        let dt = DateTime::from_rfc3339("2023-01-02T03:04:05.000+05:30").unwrap();
        let ser = serde_json::ser::to_string(&dt);
        assert!(ser.is_ok());
        assert_eq!(ser.unwrap(), "\"2023-01-01T21:34:05Z\"");
    }

    /// Test deserializing a datetime string.
    #[test]
    fn test_datetime_de_string() {
        let string_json = json!({"a": "2023-01-02T03:04:05.000Z"});
        let date: Result<DateTime, _> = serde_json::from_value(string_json["a"].clone());
        assert!(date.is_ok())
    }

    /// Test deserializing an epoch timestamp.
    #[test]
    fn test_datetime_de_number() {
        let number_json = json!({"a": 1686489749});
        let number = number_json["a"].clone();
        let date: Result<DateTime, _> = serde_json::from_value(number);
        assert_eq!(date.unwrap().to_rfc3339().unwrap(), "2023-06-11T13:22:29Z");
    }
}

//
// INTERVAL
//

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Interval {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime>,
    pub start: DateTime,
}

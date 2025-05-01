use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::{self, Visitor};
use serde::{Deserializer, Serializer};
use std::fmt;

pub mod ddnet;
pub mod ddstats;

fn serialize_datetime_timestamp<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(date.and_utc().timestamp())
}

fn deserialize_datetime_timestamp<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    struct NaiveDateTimeVisitor;

    impl<'de> Visitor<'de> for NaiveDateTimeVisitor {
        type Value = NaiveDateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or a floating point number")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value < 0 {
                return Err(E::custom("invalid timestamp: must be non-negative"));
            }

            let datetime: DateTime<Utc> =
                DateTime::from_timestamp(value, 0).unwrap_or_else(Utc::now);
            Ok(datetime.naive_utc())
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let timestamp = value as i64;
            if timestamp < 0 {
                return Err(E::custom("invalid timestamp: must be non-negative"));
            }

            let datetime: DateTime<Utc> =
                DateTime::from_timestamp(timestamp, 0).unwrap_or_else(Utc::now);
            Ok(datetime.naive_utc())
        }
    }

    deserializer.deserialize_any(NaiveDateTimeVisitor)
}

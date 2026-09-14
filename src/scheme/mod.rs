use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::{self, Visitor};
use serde::{Deserializer, Serializer};
use std::fmt;

#[cfg(feature = "ddnet")]
const DDNET_BASE_URL: &str = "ddnet.org";

#[cfg(feature = "ddstats")]
const DDSTATS_BASE_URL: &str = "ddstats.tw";

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

    impl Visitor<'_> for NaiveDateTimeVisitor {
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
            if !value.is_finite() || value < 0.0 {
                return Err(E::custom("invalid timestamp: must be non-negative"));
            }

            // JSON timestamps are integral seconds; truncation is intended.
            #[allow(clippy::cast_possible_truncation)]
            let timestamp = value as i64;

            let datetime: DateTime<Utc> =
                DateTime::from_timestamp(timestamp, 0).unwrap_or_else(Utc::now);
            Ok(datetime.naive_utc())
        }
    }

    deserializer.deserialize_any(NaiveDateTimeVisitor)
}

#[cfg(feature = "ddnet")]
pub mod ddnet;

#[cfg(feature = "ddstats")]
pub mod ddstats;

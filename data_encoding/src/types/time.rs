/// BSON date-time and timestamp types.

use std::time::{SystemTime, UNIX_EPOCH};
use crate::ser::{SerializeError, Serializer};


/* Date Time Implementation */

/// Represents a BSON UTC datetime.
#[derive(Debug, Clone, PartialEq)]
pub struct UTCDateTime {
    inner: i64,
}

impl UTCDateTime {
    /// Creates a new `UTCDateTime` from the current time.
    pub fn now() -> Self {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).unwrap();
        UTCDateTime { inner: duration.as_secs() as i64 }
    }

    /// Creates a new `UTCDateTime` from the given seconds.
    pub fn from_secs(secs: i64) -> Self {
        UTCDateTime { inner: secs }
    }

    /// Returns the seconds since the Unix epoch.
    pub fn as_secs(&self) -> i64 {
        self.inner
    }
}

impl From<i64> for UTCDateTime {
    fn from(secs: i64) -> Self {
        UTCDateTime { inner: secs }
    }
}

impl Into<i64> for UTCDateTime {
    fn into(date: UTCDateTime) -> i64 {
        date.inner
    }
}

impl From<SystemTime> for UTCDateTime {
    fn from(time: SystemTime) -> Self {
        let duration = time.duration_since(UNIX_EPOCH).unwrap();
        UTCDateTime { inner: duration.as_secs() as i64 }
    }
}

impl Into<SystemTime> for UTCDateTime {
    fn into(date: UTCDateTime) -> SystemTime {
        UNIX_EPOCH + std::time::Duration::from_secs(date.inner as u64)
    }
}

impl From<&str> for UTCDateTime {
    fn from(s: &str) -> Self {
        let secs = s.parse().unwrap();
        UTCDateTime { inner: secs }
    }
}

impl Into<String> for UTCDateTime {
    fn into(self) -> String {
        self.inner.to_string()
    }
}

impl std::fmt::Display for UTCDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/* Timestamp Implementation */

/// Represents a BSON timestamp.
#[derive(Debug, Clone, PartialEq)]
pub struct Timestamp {
    inner: i64,
}

impl Timestamp {
    /// Creates a new `Timestamp` from the current time.
    pub fn now() -> Self {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).unwrap();
        Timestamp { inner: duration.as_secs() as i64 }
    }

    /// Creates a new `Timestamp` from the given seconds.
    pub fn from_secs(secs: i64) -> Self {
        Timestamp { inner: secs }
    }

    /// Returns the seconds since the Unix epoch.
    pub fn as_secs(&self) -> i64 {
        self.inner
    }
}

impl From<i64> for Timestamp {
    fn from(secs: i64) -> Self {
        Timestamp { inner: secs }
    }
}

impl Into<i64> for Timestamp {
    fn into(date: Timestamp) -> i64 {
        date.inner
    }
}

impl From<SystemTime> for Timestamp {
    fn from(time: SystemTime) -> Self {
        let duration = time.duration_since(UNIX_EPOCH).unwrap();
        Timestamp { inner: duration.as_secs() as i64 }
    }
}

impl Into<SystemTime> for Timestamp {
    fn into(date: Timestamp) -> SystemTime {
        UNIX_EPOCH + std::time::Duration::from_secs(date.inner as u64)
    }
}

impl From<&str> for Timestamp {
    fn from(s: &str) -> Self {
        let secs = s.parse().unwrap();
        Timestamp { inner: secs }
    }
}

impl Into<String> for Timestamp {
    fn into(self) -> String {
        self.inner.to_string()
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
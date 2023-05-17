//! Utility functions that are used across several modules.

use chrono::{DateTime, Utc};

/// Converts the timestamp from the epoch used in the MPEG4 specification (seconds since 1904-01-01 00:00:00) to a
/// `DateTime` object.
#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub fn time_to_utc(time: u64) -> DateTime<Utc> {
    chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(
            mp4time_to_unix_time(time).unwrap_or_default() as i64,
            0,
        )
        .unwrap_or_default(),
        chrono::Utc,
    )
}

/// Converts the timestamp from the epoch used in the MPEG4 specification (seconds since 1904-01-01 00:00:00)
/// to the UNIX epoch time (seconds since 1970-01-01 00:00:00).
///
/// This is done by subtracting 2,082,844,800 seconds from the given time to return the new time
///  as there are 2,082,844,800 seconds from 1904-01-01 00:00:00 to 1970-01-01 00:00:00.
#[must_use]
pub const fn mp4time_to_unix_time(time: u64) -> Option<u64> {
    time.checked_sub(2_082_844_800)
}

/// Returns the duration of the presentation in seconds.
#[must_use]
pub fn duration_secs(duration: f64, timescale: f64) -> f64 {
    duration / timescale
}

/// Returns the duration in seconds as an actual duration.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn duration_seconds(duration: f64, timescale: f64) -> chrono::Duration {
    chrono::Duration::seconds(duration_secs(duration, timescale) as i64)
}

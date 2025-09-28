use std::i64;
use std::ops::{Add, Sub};

use std::time::Duration;
use std::time::SystemTime;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PostgresTimestamp(i64);

impl PostgresTimestamp {
    pub const INFINITY: PostgresTimestamp = PostgresTimestamp(i64::MAX);
    pub const NEGATIVE_INFINITY: PostgresTimestamp = PostgresTimestamp(i64::MIN);
    // Experimentally derived.
    pub const UNIX_EPOCH: PostgresTimestamp = PostgresTimestamp(-946684800000000);
}

impl Sub for PostgresTimestamp {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl Sub<Duration> for PostgresTimestamp {
    type Output = Self;

    fn sub(self, other: Duration) -> Self::Output {
        Self(self.0 - (other.as_micros() as i64))
    }
}

impl Add for PostgresTimestamp {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Add<Duration> for PostgresTimestamp {
    type Output = Self;
    fn add(self, other: Duration) -> Self::Output {
        Self(self.0 + (other.as_micros() as i64))
    }
}

impl From<SystemTime> for PostgresTimestamp {
    fn from(value: SystemTime) -> Self {
        if value < SystemTime::UNIX_EPOCH {
            let microseconds_before_unix_epoch: i64 = SystemTime::UNIX_EPOCH
                .duration_since(value)
                .unwrap()
                .as_micros() as i64;
            PostgresTimestamp::UNIX_EPOCH - PostgresTimestamp(microseconds_before_unix_epoch)
        } else if value > SystemTime::UNIX_EPOCH {
            let microseconds_after_unix_epoch: i64 = value
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_micros() as i64;
            PostgresTimestamp::UNIX_EPOCH + PostgresTimestamp(microseconds_after_unix_epoch)
        } else {
            PostgresTimestamp::UNIX_EPOCH
        }
    }
}

impl From<PostgresTimestamp> for SystemTime {
    fn from(value: PostgresTimestamp) -> Self {
        if value < PostgresTimestamp::UNIX_EPOCH {
            let microseconds_before_unix_epoch: i64 = (PostgresTimestamp::UNIX_EPOCH - value).0;
            assert!(microseconds_before_unix_epoch.is_positive());
            let microseconds_before_unix_epoch: u64 = microseconds_before_unix_epoch as u64;
            let microseconds_before_unix_epoch: Duration =
                Duration::from_micros(microseconds_before_unix_epoch);
            SystemTime::UNIX_EPOCH - microseconds_before_unix_epoch
        } else if value > PostgresTimestamp::UNIX_EPOCH {
            let microseconds_after_unix_epoch: i64 = (value - PostgresTimestamp::UNIX_EPOCH).0;
            assert!(microseconds_after_unix_epoch.is_positive());
            let microseconds_after_unix_epoch: u64 = microseconds_after_unix_epoch as u64;
            let microseconds_after_unix_epoch: Duration =
                Duration::from_micros(microseconds_after_unix_epoch);
            SystemTime::UNIX_EPOCH + microseconds_after_unix_epoch
        } else {
            SystemTime::UNIX_EPOCH
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const ONE_HUNDRED_DAYS_IN_SECONDS: u64 = 8640000;
    #[test]
    fn test_from_systemtime() {
        assert_eq!(
            PostgresTimestamp::UNIX_EPOCH,
            PostgresTimestamp::from(SystemTime::UNIX_EPOCH)
        );
        assert_eq!(
            PostgresTimestamp::UNIX_EPOCH + Duration::from_secs(ONE_HUNDRED_DAYS_IN_SECONDS),
            PostgresTimestamp::from(
                SystemTime::UNIX_EPOCH + Duration::from_secs(ONE_HUNDRED_DAYS_IN_SECONDS)
            )
        );
        assert_eq!(
            PostgresTimestamp::UNIX_EPOCH - Duration::from_secs(ONE_HUNDRED_DAYS_IN_SECONDS),
            PostgresTimestamp::from(
                SystemTime::UNIX_EPOCH - Duration::from_secs(ONE_HUNDRED_DAYS_IN_SECONDS)
            )
        );
    }
    #[test]
    fn test_into_systemtime() {
        assert_eq!(
            SystemTime::from(PostgresTimestamp::UNIX_EPOCH),
            SystemTime::UNIX_EPOCH
        );
        assert_eq!(
            SystemTime::from(
                PostgresTimestamp::UNIX_EPOCH + Duration::from_secs(ONE_HUNDRED_DAYS_IN_SECONDS)
            ),
            SystemTime::UNIX_EPOCH + Duration::from_secs(ONE_HUNDRED_DAYS_IN_SECONDS)
        );
        assert_eq!(
            SystemTime::from(
                PostgresTimestamp::UNIX_EPOCH - Duration::from_secs(ONE_HUNDRED_DAYS_IN_SECONDS)
            ),
            SystemTime::UNIX_EPOCH - Duration::from_secs(ONE_HUNDRED_DAYS_IN_SECONDS)
        );
    }
}

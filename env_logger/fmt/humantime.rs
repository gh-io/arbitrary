use core::fmt;
use std::time::SystemTime;

/* =========================
   Timestamp Precision
   ========================= */

#[derive(Debug, Clone, Copy)]
pub enum TimestampPrecision {
    Seconds,
    Millis,
    Micros,
    Nanos,
}

/* =========================
   Timestamp Type
   ========================= */

pub struct Timestamp {
    time: SystemTime,
    precision: TimestampPrecision,
}

impl Timestamp {
    /// Create a timestamp for `now` with chosen precision
    pub fn now(precision: TimestampPrecision) -> Self {
        Self {
            time: SystemTime::now(),
            precision,
        }
    }
}

/* =========================
   Display + Debug (RFC3339)
   ========================= */

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Ok(ts) = jiff::Timestamp::try_from(self.time) else {
            return Err(fmt::Error);
        };

        match self.precision {
            TimestampPrecision::Seconds => write!(f, "{ts:.0}"),
            TimestampPrecision::Millis => write!(f, "{ts:.3}"),
            TimestampPrecision::Micros => write!(f, "{ts:.6}"),
            TimestampPrecision::Nanos => write!(f, "{ts:.9}"),
        }
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/* =========================
   ENV Precision Support
   ========================= */

pub fn precision_from_env() -> TimestampPrecision {
    match std::env::var("LOG_TS_PRECISION").as_deref() {
        Ok("nanos") => TimestampPrecision::Nanos,
        Ok("micros") => TimestampPrecision::Micros,
        Ok("millis") => TimestampPrecision::Millis,
        _ => TimestampPrecision::Seconds,
    }
}

/* =========================
   env_logger Integration
   ========================= */

pub fn init_env_logger() {
    use std::io::Write;

    env_logger::Builder::new()
        .format(|buf, record| {
            let ts = Timestamp::now(precision_from_env());
            writeln!(
                buf,
                "{} [{}] {}",
                ts,
                record.level(),
                record.args()
            )
        })
        .init();
}

/* =========================
   tracing Integration
   ========================= */

#[cfg(feature = "tracing")]
pub mod tracing_time {
    use super::*;
    use tracing_subscriber::fmt::time::FormatTime;

    pub struct Rfc3339Time;

    impl FormatTime for Rfc3339Time {
        fn format_time(&self, w: &mut dyn fmt::Write) -> fmt::Result {
            write!(w, "{}", Timestamp::now(TimestampPrecision::Millis))
        }
    }
}

/* =========================
   Tests
   ========================= */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch_formats() {
        let mut ts = Timestamp {
            time: SystemTime::UNIX_EPOCH,
            precision: TimestampPrecision::Nanos,
        };

        assert_eq!("1970-01-01T00:00:00.000000000Z", format!("{ts}"));

        ts.precision = TimestampPrecision::Micros;
        assert_eq!("1970-01-01T00:00:00.000000Z", format!("{ts}"));

        ts.precision = TimestampPrecision::Millis;
        assert_eq!("1970-01-01T00:00:00.000Z", format!("{ts}"));

        ts.precision = TimestampPrecision::Seconds;
        assert_eq!("1970-01-01T00:00:00Z", format!("{ts}"));
    }
}

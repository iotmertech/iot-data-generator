use crate::config::model::{TimeConfig, TimeMode};
use crate::error::{MerError, Result};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rand::Rng;

/// A simulated clock that produces the timestamp for each generated message.
///
/// The clock is shared across all devices: it advances once per message in the
/// same order messages are produced (matching the global `seq` counter).
pub struct SimClock {
    mode: TimeMode,
    start: DateTime<Utc>,
    step_secs: i64,
    min_secs: i64,
    max_secs: i64,
    /// Accumulated time for `random` mode.
    current: DateTime<Utc>,
    field: String,
}

impl SimClock {
    pub fn from_config(cfg: &Option<TimeConfig>) -> Result<Self> {
        let cfg = match cfg {
            Some(c) => c.clone(),
            None => {
                return Ok(Self {
                    mode: TimeMode::Real,
                    start: Utc::now(),
                    step_secs: 0,
                    min_secs: 0,
                    max_secs: 0,
                    current: Utc::now(),
                    field: "ts".to_string(),
                })
            }
        };

        let start = match &cfg.start {
            Some(s) => parse_start(s)?,
            None => Utc::now(),
        };

        Ok(Self {
            mode: cfg.mode,
            start,
            step_secs: cfg.step_secs.unwrap_or(0),
            min_secs: cfg.min_secs.unwrap_or(0),
            max_secs: cfg.max_secs.unwrap_or(0),
            current: start,
            field: cfg.field,
        })
    }

    /// Field name to write the timestamp into (random payload mode).
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Returns the timestamp for the message with the given global sequence
    /// number. Must be called once per message, in order.
    pub fn timestamp(&mut self, seq: usize) -> DateTime<Utc> {
        match self.mode {
            TimeMode::Real => Utc::now(),
            TimeMode::Fixed => self.start + chrono::Duration::seconds(self.step_secs * seq as i64),
            TimeMode::Random => {
                let ts = self.current;
                let step = if self.max_secs > self.min_secs {
                    rand::thread_rng().gen_range(self.min_secs..=self.max_secs)
                } else {
                    self.min_secs
                };
                self.current += chrono::Duration::seconds(step);
                ts
            }
        }
    }
}

/// Parse a start instant. Accepts RFC3339 (`2026-01-01T00:00:00Z`) and the
/// friendlier `2026-01-01 00:00:00` form (interpreted as UTC).
fn parse_start(s: &str) -> Result<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }
    if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Ok(Utc.from_utc_datetime(&naive));
    }
    Err(MerError::Config(format!(
        "Invalid time.start '{s}': expected RFC3339 (2026-01-01T00:00:00Z) or 'YYYY-MM-DD HH:MM:SS'"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::model::TimeConfig;

    fn cfg(mode: TimeMode, start: &str, step: i64, min: i64, max: i64) -> Option<TimeConfig> {
        Some(TimeConfig {
            mode,
            start: Some(start.to_string()),
            step_secs: Some(step),
            min_secs: Some(min),
            max_secs: Some(max),
            field: "ts".to_string(),
        })
    }

    #[test]
    fn test_real_mode_defaults_to_now() {
        let mut clock = SimClock::from_config(&None).unwrap();
        let before = Utc::now();
        let ts = clock.timestamp(0);
        let after = Utc::now();
        assert!(ts >= before && ts <= after);
    }

    #[test]
    fn test_fixed_mode_is_evenly_spaced() {
        let mut clock =
            SimClock::from_config(&cfg(TimeMode::Fixed, "2026-01-01T00:00:00Z", 300, 0, 0))
                .unwrap();
        assert_eq!(clock.timestamp(0).to_rfc3339(), "2026-01-01T00:00:00+00:00");
        assert_eq!(clock.timestamp(1).to_rfc3339(), "2026-01-01T00:05:00+00:00");
        assert_eq!(clock.timestamp(2).to_rfc3339(), "2026-01-01T00:10:00+00:00");
    }

    #[test]
    fn test_random_mode_accumulates_within_bounds() {
        let mut clock =
            SimClock::from_config(&cfg(TimeMode::Random, "2026-01-01 00:00:00", 0, 60, 1800))
                .unwrap();
        let mut prev = clock.timestamp(0);
        assert_eq!(prev.to_rfc3339(), "2026-01-01T00:00:00+00:00");
        for i in 1..20 {
            let next = clock.timestamp(i);
            let diff = (next - prev).num_seconds();
            assert!((60..=1800).contains(&diff), "step out of range: {diff}");
            prev = next;
        }
    }

    #[test]
    fn test_naive_start_parsed_as_utc() {
        let mut clock =
            SimClock::from_config(&cfg(TimeMode::Fixed, "2026-01-01 12:30:00", 0, 0, 0)).unwrap();
        assert_eq!(clock.timestamp(0).to_rfc3339(), "2026-01-01T12:30:00+00:00");
    }

    #[test]
    fn test_invalid_start_errors() {
        let result = SimClock::from_config(&cfg(TimeMode::Fixed, "not-a-date", 0, 0, 0));
        assert!(result.is_err());
    }
}

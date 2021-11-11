use std::fmt;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

/// A stateful timer implementation.
#[derive(Default, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct Timer {
    /// The total duration of this timer
    #[serde(with = "serde_millis")]
    duration: Duration,
    /// The amount of time which has been accumulated.
    /// This only increases when the timer has been paused.
    #[serde(with = "serde_millis")]
    accumulated_time: Duration,
    /// The time at which this timer was started.
    #[serde(with = "serde_millis")]
    start_timestamp: Option<Instant>,
}

impl Timer {
    pub fn from_duration(duration: Duration) -> Self {
        Self {
            duration,
            accumulated_time: Duration::ZERO,
            start_timestamp: None,
        }
    }

    pub fn start(&mut self) {
        if self.start_timestamp.is_none() {
            self.start_timestamp = Some(Instant::now());
        }
    }

    pub fn pause(&mut self) {
        if let Some(start_timestamp) = self.start_timestamp {
            let elapsed = Instant::now().duration_since(start_timestamp);
            self.accumulated_time += elapsed;
        }

        self.start_timestamp = None;
    }

    pub fn set_duration(&mut self, new_duration: Duration) {
        self.duration = new_duration;
    }

    pub fn elapsed(&self) -> Duration {
        let current_elapsed = match self.start_timestamp {
            Some(start_timestamp) => {
                Instant::now().duration_since(start_timestamp)
            }
            None => Duration::ZERO,
        };
        self.accumulated_time + current_elapsed
    }

    pub fn remaining_time(&self) -> Duration {
        return self.duration.saturating_sub(self.elapsed());
    }

    pub fn has_started(&self) -> bool {
        return self.elapsed() > Duration::ZERO;
    }

    pub fn is_over(&self) -> bool {
        return self.elapsed() >= self.duration;
    }

    pub fn is_running(&self) -> bool {
        self.start_timestamp.is_some()
    }

    pub fn is_paused(&self) -> bool {
        self.start_timestamp.is_none() && self.has_started()
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_seconds = self.remaining_time().as_secs();
        write!(f, "{:02}:{:02}", total_seconds / 60, total_seconds % 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        // test basic operations

        let t1 = Timer::from_duration(Duration::ZERO);

        assert!(!t1.is_paused());
        assert!(!t1.is_running());
        assert!(!t1.has_started());

        let mut t2 = Timer {
            duration: Duration::from_secs(20),
            accumulated_time: Duration::from_secs(12),
            start_timestamp: None,
        };

        assert!(t2.is_paused());
        assert!(!t2.is_running());
        assert!(t2.has_started());
        assert_eq!(Duration::from_secs(12), t2.elapsed());
        assert_eq!(Duration::from_secs(8), t2.remaining_time());

        t2.start();
        assert!(!t2.is_paused());
        assert!(t2.is_running());
        assert!(t2.has_started());

        // test formatting
        assert_eq!("00:00", format!("{}", Timer::default()));
        assert_eq!(
            "12:34",
            format!("{}", Timer::from_duration(Duration::from_secs(754)))
        );
    }
}

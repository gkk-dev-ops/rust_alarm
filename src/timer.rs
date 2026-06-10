use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub struct Countdown {
    started_at: Instant,
    duration: Duration,
}

impl Countdown {
    pub fn new(started_at: Instant, duration: Duration) -> Self {
        Self {
            started_at,
            duration,
        }
    }

    pub fn remaining(&self, now: Instant) -> Duration {
        self.duration
            .saturating_sub(now.duration_since(self.started_at))
    }

    pub fn is_finished(&self, now: Instant) -> bool {
        self.remaining(now).is_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::Countdown;
    use std::time::{Duration, Instant};

    #[test]
    fn remaining_time_never_underflows() {
        let start = Instant::now();
        let timer = Countdown::new(start, Duration::from_secs(90));
        assert_eq!(
            timer.remaining(start + Duration::from_secs(30)),
            Duration::from_secs(60)
        );
        assert_eq!(
            timer.remaining(start + Duration::from_secs(100)),
            Duration::ZERO
        );
    }
}

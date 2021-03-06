use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct Timer {
    begin: Instant,
    duration: Duration,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            begin: Instant::now(),
            duration,
        }
    }
    pub fn from_millis(millis: u64) -> Self {
        Self::new(Duration::from_millis(millis))
    }
    /// `duration` を `1.0` として
    pub fn now(&self) -> f64 {
        (Instant::now() - self.begin).as_secs_f64() / self.duration.as_secs_f64()
    }
    pub fn end(&self) -> bool {
        Instant::now() >= self.begin + self.duration
    }
    /// 現在から、「このタイマーの `begin` から `duration` 後」までのタイマー
    pub fn subtimer(&self, duration: Duration) -> Self {
        let now = Instant::now();
        Self {
            begin: now,
            duration: duration - (now - self.begin),
        }
    }
}

#[cfg(test)]
mod test {}
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct Timer {
    begin: Instant,
    duration: Duration,
}

impl Timer {
    pub fn new(millis: u64) -> Self {
        Self {
            begin: Instant::now(),
            duration: Duration::from_millis(millis),
        }
    }
    pub fn from_to(from: Instant, to: Instant) -> Self {
        Self {
            begin: from,
            duration: to - from,
        }
    }
    /// `duration` を `1.0` として
    pub fn now(&self) -> f64 {
        (Instant::now() - self.begin).as_secs_f64() / self.duration.as_secs_f64()
    }
    pub fn end(&self) -> bool {
        Instant::now() >= self.begin + self.duration
    }
    /// 現在から、「このタイマーの `begin` から `ration * duration` 後」までのタイマー
    pub fn subtimer(&self, ratio: f64) -> Self {
        let now = Instant::now();
        assert!(ratio <= 1.0, "too long timer");
        Self::from_to(now, self.begin + self.duration.mul_f64(ratio))
    }
}

#[cfg(test)]
mod test {}

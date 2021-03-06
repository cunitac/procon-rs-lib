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
    /// `duration` を `1.0` として
    pub fn now(&self) -> f64 {
        (Instant::now() - self.begin).as_secs_f64() / self.duration.as_secs_f64()
    }
    pub fn end(&self) -> bool {
        Instant::now() >= self.begin + self.duration
    }
    /// 現在から、「このタイマーの `begin` から `duration` 後」までのタイマー
    pub fn subtimer(&self, millis: u64) -> Self {
        let now = Instant::now();
        Self {
            begin: now,
            duration: Duration::from_millis(millis) - (now - self.begin),
        }
    }
}

#[cfg(test)]
mod test {}

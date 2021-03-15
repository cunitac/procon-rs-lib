use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct Timer {
    begin: Instant,
    duration: Duration,
}

impl Timer {
    pub fn from_millis(millis: u64) -> Self {
        Self {
            begin: Instant::now(),
            duration: Duration::from_millis(millis),
        }
    }
    /// `duration` を `1.0` として
    pub fn elapsed(&self) -> f64 {
        (Instant::now() - self.begin).as_secs_f64() / self.duration.as_secs_f64()
    }
    pub fn is_over(&self) -> bool {
        Instant::now() - self.begin > self.duration
    }
    pub fn freeze(self) -> FreezedTimer {
        FreezedTimer {
            freezed: Instant::now(),
            timer: self,
        }
    }
    /// 現在から `duration * ratio` 後までの `Timer`
    /// `self` 終了後も続くとパニック
    pub fn subtimer(&self, ratio: f64) -> Self {
        let begin = Instant::now();
        let duration = self.duration.mul_f64(ratio);
        assert!(
            self.begin + self.duration >= begin + duration,
            "too long subtimer, elapsed = {:.3}, ratio = {:.3}",
            self.elapsed(),
            ratio
        );
        Self { begin, duration }
    }
}

#[derive(Clone, Copy)]
pub struct FreezedTimer {
    freezed: Instant,
    timer: Timer,
}

impl FreezedTimer {
    pub fn restart(self) -> Timer {
        Timer {
            begin: self.timer.begin + (Instant::now() - self.freezed),
            duration: self.timer.duration,
        }
    }
}

#[cfg(test)]
mod test {}

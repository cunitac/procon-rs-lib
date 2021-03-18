use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct Timer {
    begin: Instant,
    duration: Duration,
}

impl Timer {
    pub fn new(secs: f64) -> Self {
        Self {
            begin: Instant::now(),
            duration: Duration::from_secs_f64(secs),
        }
    }
    /// `duration` を `1.0` として
    pub fn elapsed(&self) -> f64 {
        self.elapsed_duration().as_secs_f64() / self.duration.as_secs_f64()
    }
    pub fn is_over(&self) -> bool {
        self.elapsed_duration() > self.duration
    }
    pub fn freeze(self) -> FreezedTimer {
        FreezedTimer {
            freezed: Instant::now(),
            timer: self,
        }
    }
    /// 現在から、「`begin` から `secs` 秒後」まで
    pub fn subtimer(&self, secs: f64) -> Self {
        Self {
            begin: Instant::now(),
            duration: Duration::from_secs_f64(secs) - self.elapsed_duration(),
        }
    }
    fn elapsed_duration(&self) -> Duration {
        Instant::now() - self.begin
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

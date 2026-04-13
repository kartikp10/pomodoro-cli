use crate::types::{SessionType, TimerState};
use std::time::{Duration, Instant};

/// Core timer that tracks elapsed time and remaining seconds.
pub struct Timer {
    pub session_type: SessionType,
    pub total_secs: u64,
    pub state: TimerState,
    elapsed_before_pause: Duration,
    last_resumed: Instant,
}

impl Timer {
    pub fn new(session_type: SessionType, total_secs: u64) -> Self {
        Self {
            session_type,
            total_secs,
            state: TimerState::Running,
            elapsed_before_pause: Duration::ZERO,
            last_resumed: Instant::now(),
        }
    }

    /// Seconds remaining (clamped to 0).
    pub fn remaining_secs(&self) -> u64 {
        let elapsed = match self.state {
            TimerState::Running => self.elapsed_before_pause + self.last_resumed.elapsed(),
            TimerState::Paused | TimerState::Finished => self.elapsed_before_pause,
        };
        self.total_secs.saturating_sub(elapsed.as_secs())
    }

    /// Fraction complete in [0.0, 1.0].
    pub fn progress(&self) -> f64 {
        if self.total_secs == 0 {
            return 1.0;
        }
        let remaining = self.remaining_secs() as f64;
        1.0 - (remaining / self.total_secs as f64)
    }

    /// Check if the timer has reached zero; if so, transition to Finished.
    pub fn tick(&mut self) -> bool {
        if self.state == TimerState::Running && self.remaining_secs() == 0 {
            self.state = TimerState::Finished;
        }
        self.state == TimerState::Finished
    }

    pub fn pause(&mut self) {
        if self.state == TimerState::Running {
            self.elapsed_before_pause += self.last_resumed.elapsed();
            self.state = TimerState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.state == TimerState::Paused {
            self.last_resumed = Instant::now();
            self.state = TimerState::Running;
        }
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            TimerState::Running => self.pause(),
            TimerState::Paused => self.resume(),
            TimerState::Finished => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_new_timer() {
        let t = Timer::new(SessionType::Work, 60);
        assert_eq!(t.state, TimerState::Running);
        assert!(t.remaining_secs() <= 60);
    }

    #[test]
    fn test_short_timer_finishes() {
        let mut t = Timer::new(SessionType::Work, 1);
        thread::sleep(Duration::from_millis(1100));
        assert!(t.tick());
        assert_eq!(t.state, TimerState::Finished);
    }

    #[test]
    fn test_pause_resume() {
        let mut t = Timer::new(SessionType::Work, 300);
        t.pause();
        assert_eq!(t.state, TimerState::Paused);
        let r1 = t.remaining_secs();
        thread::sleep(Duration::from_millis(200));
        // Time should not advance while paused
        assert_eq!(t.remaining_secs(), r1);
        t.resume();
        assert_eq!(t.state, TimerState::Running);
    }
}

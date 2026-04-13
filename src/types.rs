use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Type of Pomodoro session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionType {
    Work,
    ShortBreak,
    LongBreak,
}

impl fmt::Display for SessionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionType::Work => write!(f, "🍅 Work"),
            SessionType::ShortBreak => write!(f, "☕ Short Break"),
            SessionType::LongBreak => write!(f, "🌴 Long Break"),
        }
    }
}

/// Runtime state of the timer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Running,
    Paused,
    Finished,
}

/// A completed session record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_type: SessionType,
    pub duration_secs: u64,
    pub started_at: DateTime<Local>,
    pub completed_at: DateTime<Local>,
}

/// User configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub work_mins: u64,
    pub short_break_mins: u64,
    pub long_break_mins: u64,
    pub rounds: u32,
    pub notify: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_mins: 25,
            short_break_mins: 5,
            long_break_mins: 15,
            rounds: 4,
            notify: true,
        }
    }
}

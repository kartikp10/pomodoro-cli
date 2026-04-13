use crate::config::data_dir;
use crate::types::Session;
use std::fs;

fn log_path() -> std::path::PathBuf {
    data_dir().join("log.json")
}

/// Read all logged sessions.
pub fn read_sessions() -> Vec<Session> {
    let path = log_path();
    if !path.exists() {
        return Vec::new();
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

/// Append a completed session to the log.
pub fn log_session(session: &Session) {
    let mut sessions = read_sessions();
    sessions.push(session.clone());
    let json = serde_json::to_string_pretty(&sessions).expect("Failed to serialize sessions");
    fs::write(log_path(), json).expect("Failed to write session log");
}

/// Clear the session log.
pub fn clear_log() {
    let path = log_path();
    if path.exists() {
        fs::remove_file(&path).expect("Failed to remove session log");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SessionType;
    use chrono::Local;

    #[test]
    fn test_log_roundtrip() {
        // Use a temp approach: just verify serialization
        let session = Session {
            session_type: SessionType::Work,
            duration_secs: 1500,
            started_at: Local::now(),
            completed_at: Local::now(),
        };
        let json = serde_json::to_string(&session).unwrap();
        let parsed: Session = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.session_type, SessionType::Work);
        assert_eq!(parsed.duration_secs, 1500);
    }
}

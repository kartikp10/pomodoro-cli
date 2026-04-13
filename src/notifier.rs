use crate::types::SessionType;

/// Send a desktop notification when a session ends.
pub fn notify_session_end(session_type: SessionType) {
    let (title, body) = match session_type {
        SessionType::Work => ("Work session complete!", "Time for a break."),
        SessionType::ShortBreak => ("Break over!", "Time to focus."),
        SessionType::LongBreak => ("Long break over!", "Ready for a new cycle."),
    };

    // Terminal bell as fallback
    print!("\x07");

    #[cfg(not(test))]
    {
        let _ = notify_rust::Notification::new()
            .summary(title)
            .body(body)
            .timeout(5000)
            .show();
    }

    #[cfg(test)]
    {
        let _ = (title, body);
    }
}

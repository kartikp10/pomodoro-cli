use crate::types::SessionType;

// Embedded tomato icon for notifications.
// PNG works on macOS + Linux; SVG only works on Linux.
#[cfg(not(test))]
const TOMATO_PNG: &[u8] = include_bytes!("../assets/tomato.png");

/// Write the embedded icon to ~/.pomodoro-cli/ and return its path.
#[cfg(not(test))]
fn icon_path() -> String {
    let dir = crate::config::data_dir();
    let path = dir.join("tomato.png");
    if !path.exists() {
        let _ = std::fs::write(&path, TOMATO_PNG);
    }
    path.to_string_lossy().to_string()
}

/// Send a desktop notification with sound and icon when a session ends.
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
        let mut notification = notify_rust::Notification::new();
        notification
            .summary(title)
            .body(body)
            .timeout(5000)
            .image_path(&icon_path());

        // macOS: play a sound
        #[cfg(target_os = "macos")]
        {
            notification.sound_name("Glass");
        }

        // Linux: sound hint (icon already set via image_path → image-path hint)
        #[cfg(target_os = "linux")]
        {
            notification.icon(&icon_path());
            notification.hint(notify_rust::Hint::SoundName("complete".to_string()));
        }

        let _ = notification.show();
    }

    #[cfg(test)]
    {
        let _ = (title, body);
    }
}

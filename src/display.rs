use crate::timer::Timer;
use crate::types::{SessionType, TimerState};
use crossterm::{cursor, execute, style::Print, terminal};
use std::io::{self, Write};

const BAR_WIDTH: usize = 30;

// ANSI color helpers
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";

const RED: &str = "\x1b[38;5;203m";
const GREEN: &str = "\x1b[38;5;114m";
const BLUE: &str = "\x1b[38;5;110m";
const YELLOW: &str = "\x1b[38;5;222m";
const GRAY: &str = "\x1b[38;5;242m";
const WHITE: &str = "\x1b[38;5;255m";

/// Pick the accent color for the current session type.
fn accent(session_type: SessionType) -> &'static str {
    match session_type {
        SessionType::Work => RED,
        SessionType::ShortBreak => GREEN,
        SessionType::LongBreak => BLUE,
    }
}

/// Visible length of a string, ignoring ANSI escape sequences.
fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c.is_ascii_alphabetic() {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            // Some emoji/symbols are double-width
            len += unicode_width(c);
        }
    }
    len
}

/// Approximate character display width (ASCII = 1, most emoji/CJK = 2).
fn unicode_width(c: char) -> usize {
    // Common emoji and symbols used in our UI
    if matches!(c, '🍅' | '☕' | '🌴' | '⏸') {
        2
    } else if c.is_ascii() {
        1
    } else if ('\u{1100}'..='\u{115F}').contains(&c)
        || ('\u{2E80}'..='\u{9FFF}').contains(&c)
        || ('\u{F900}'..='\u{FAFF}').contains(&c)
        || ('\u{FE10}'..='\u{FE6F}').contains(&c)
        || ('\u{FF00}'..='\u{FF60}').contains(&c)
        || ('\u{1F300}'..='\u{1F9FF}').contains(&c)
    {
        2
    } else {
        1
    }
}

/// Center-pad a line to fit the terminal width.
fn center(line: &str, term_width: usize) -> String {
    let vis = visible_len(line);
    if vis >= term_width {
        return line.to_string();
    }
    let pad = (term_width - vis) / 2;
    format!("{}{}", " ".repeat(pad), line)
}

/// Render a single frame of the timer display, centered in the terminal.
pub fn render(timer: &Timer, round: u32, total_rounds: u32) -> io::Result<()> {
    let mut stdout = io::stdout();
    let (term_w, term_h) = terminal::size().unwrap_or((80, 24));
    let term_width = term_w as usize;
    let term_height = term_h as usize;

    execute!(
        stdout,
        cursor::Hide,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All),
    )?;

    let remaining = timer.remaining_secs();
    let mins = remaining / 60;
    let secs = remaining % 60;
    let progress = timer.progress();
    let color = accent(timer.session_type);

    // Progress bar with colored fill
    let filled = (progress * BAR_WIDTH as f64) as usize;
    let empty = BAR_WIDTH - filled;
    let bar = format!(
        "{color}{}{RESET}{GRAY}{}{RESET}",
        "█".repeat(filled),
        "▒".repeat(empty),
    );

    // State badge
    let state_badge = match timer.state {
        TimerState::Running => String::new(),
        TimerState::Paused => format!("  {YELLOW}{BOLD}⏸ PAUSED{RESET}"),
        TimerState::Finished => format!("  {GREEN}{BOLD}✓ DONE{RESET}"),
    };

    // Round dots: filled for completed, empty for remaining
    let dots: String = (1..=total_rounds)
        .map(|i| {
            if i <= round {
                format!("{color}●{RESET}")
            } else {
                format!("{GRAY}○{RESET}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    // Percentage label with balanced padding so the bar stays centered
    let pct_label = format!("{:.0}%", progress * 100.0);
    let label_max_width = 4; // "100%"
    let label_pad = label_max_width - pct_label.len();
    let bar_line = format!(
        "{}{DIM}{bar}{RESET}  {GRAY}{}{}{RESET}",
        " ".repeat(label_max_width + 2),
        pct_label,
        " ".repeat(label_pad),
    );

    // Build content lines
    let lines: Vec<String> = vec![
        String::new(),
        format!("{BOLD}{color}{}{RESET}", timer.session_type),
        String::new(),
        format!(
            "{GRAY}round {WHITE}{}{GRAY}/{}{RESET}   {}",
            round, total_rounds, dots
        ),
        String::new(),
        String::new(),
        format!("{BOLD}{WHITE}{:02}:{:02}{RESET}{}", mins, secs, state_badge),
        String::new(),
        bar_line,
        String::new(),
        String::new(),
        format!("{DIM}{GRAY}p {WHITE}pause{GRAY}  ·  s {WHITE}skip{GRAY}  ·  q {WHITE}quit{RESET}"),
    ];

    // Vertical centering: pad top so content block is in the middle
    let content_height = lines.len();
    let top_pad = if term_height > content_height {
        (term_height - content_height) / 2
    } else {
        0
    };

    // Build the full frame
    let mut frame = String::new();
    for _ in 0..top_pad {
        frame.push_str("\r\n");
    }
    for line in &lines {
        frame.push_str(&center(line, term_width));
        frame.push_str("\r\n");
    }

    execute!(stdout, Print(frame))?;
    stdout.flush()?;
    Ok(())
}

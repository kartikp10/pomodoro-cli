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

/// Render a single frame of the timer display.
pub fn render(timer: &Timer, round: u32, total_rounds: u32) -> io::Result<()> {
    let mut stdout = io::stdout();

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

    let display = format!(
        "\r\n\
         \r\n\
         {BOLD}{color}   {}{RESET}\r\n\
         \r\n\
         {GRAY}   round {WHITE}{}{GRAY}/{}{RESET}   {}\r\n\
         \r\n\
         \r\n\
         {BOLD}{WHITE}   {:02}:{:02}{RESET}{}\r\n\
         \r\n\
         {DIM}   {bar}{RESET}  {GRAY}{:.0}%{RESET}\r\n\
         \r\n\
         \r\n\
         {DIM}{GRAY}   p {WHITE}pause{GRAY}  ·  s {WHITE}skip{GRAY}  ·  q {WHITE}quit{RESET}\r\n",
        timer.session_type,
        round,
        total_rounds,
        dots,
        mins,
        secs,
        state_badge,
        progress * 100.0,
    );

    execute!(stdout, Print(display))?;
    stdout.flush()?;
    Ok(())
}

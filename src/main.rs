mod config;
mod display;
mod logger;
mod notifier;
mod stats;
mod timer;
mod types;

use crate::config::{load_config, merge_overrides};
use crate::display::render;
use crate::logger::log_session;
use crate::notifier::notify_session_end;
use crate::timer::Timer;
use crate::types::{Config, Session, SessionType};
use chrono::Local;
use clap::{Parser, Subcommand};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "pomodoro", about = "A command-line Pomodoro timer 🍅")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Work duration in minutes
    #[arg(long, global = true)]
    work: Option<u64>,

    /// Short break duration in minutes
    #[arg(long, global = true)]
    short_break: Option<u64>,

    /// Long break duration in minutes
    #[arg(long, global = true)]
    long_break: Option<u64>,

    /// Number of work rounds before a long break
    #[arg(long, global = true)]
    rounds: Option<u32>,

    /// Disable desktop notifications
    #[arg(long, global = true, default_value_t = false)]
    no_notify: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Start a Pomodoro session (default)
    Start,
    /// Show session statistics
    Stats,
    /// Print current configuration
    Config,
    /// Clear session history
    Reset,
}

fn main() {
    let cli = Cli::parse();
    let cfg = merge_overrides(
        load_config(),
        cli.work,
        cli.short_break,
        cli.long_break,
        cli.rounds,
        cli.no_notify,
    );

    match cli.command {
        None | Some(Command::Start) => run_pomodoro(&cfg),
        Some(Command::Stats) => stats::show_stats(),
        Some(Command::Config) => print_config(&cfg),
        Some(Command::Reset) => {
            logger::clear_log();
            println!("  Session history cleared.");
        }
    }
}

fn print_config(cfg: &Config) {
    println!();
    println!("  ⚙️  Current Configuration");
    println!("  ─────────────────────────────");
    println!("  Work:        {} min", cfg.work_mins);
    println!("  Short break: {} min", cfg.short_break_mins);
    println!("  Long break:  {} min", cfg.long_break_mins);
    println!("  Rounds:      {}", cfg.rounds);
    println!("  Notify:      {}", cfg.notify);
    println!();
}

fn run_pomodoro(cfg: &Config) {
    // Check if we have a real terminal
    if !std::io::IsTerminal::is_terminal(&std::io::stdout()) {
        eprintln!("Error: pomodoro requires an interactive terminal.");
        std::process::exit(1);
    }

    // Enable raw mode for keyboard input
    if let Err(e) = terminal::enable_raw_mode() {
        eprintln!("Error: Failed to enable raw mode: {e}");
        eprintln!("Make sure you're running in an interactive terminal.");
        std::process::exit(1);
    }
    let result = run_pomodoro_loop(cfg);
    let _ = terminal::disable_raw_mode();

    // Clear screen and show cursor on exit
    let _ = crossterm::execute!(
        std::io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 0),
        crossterm::cursor::Show,
    );

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}

fn run_pomodoro_loop(cfg: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut work_count: u32 = 0;

    loop {
        // --- Work session ---
        work_count += 1;
        let secs = cfg.work_mins * 60;
        let action = run_single_timer(SessionType::Work, secs, work_count, cfg.rounds)?;
        if action == Action::Quit {
            break;
        }
        if action == Action::Completed && cfg.notify {
            notify_session_end(SessionType::Work);
        }
        if action == Action::Completed {
            log_completed(SessionType::Work, secs);
        }

        // --- Break session ---
        let (break_type, break_secs) = if work_count >= cfg.rounds {
            work_count = 0;
            (SessionType::LongBreak, cfg.long_break_mins * 60)
        } else {
            (SessionType::ShortBreak, cfg.short_break_mins * 60)
        };

        let action = run_single_timer(break_type, break_secs, work_count, cfg.rounds)?;
        if action == Action::Quit {
            break;
        }
        if action == Action::Completed && cfg.notify {
            notify_session_end(break_type);
        }
        if action == Action::Completed {
            log_completed(break_type, break_secs);
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Action {
    Completed,
    Skipped,
    Quit,
}

fn run_single_timer(
    session_type: SessionType,
    total_secs: u64,
    round: u32,
    total_rounds: u32,
) -> Result<Action, Box<dyn std::error::Error>> {
    let mut timer = Timer::new(session_type, total_secs);

    loop {
        render(&timer, round, total_rounds)?;

        // Poll for keyboard input (200ms timeout for smooth updates)
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(Action::Quit),
                    KeyCode::Char('s') => return Ok(Action::Skipped),
                    KeyCode::Char('p') | KeyCode::Char(' ') => timer.toggle_pause(),
                    _ => {}
                }
            }
        }

        if timer.tick() {
            render(&timer, round, total_rounds)?;
            // Brief pause so the user sees 00:00
            std::thread::sleep(Duration::from_secs(1));
            return Ok(Action::Completed);
        }
    }
}

fn log_completed(session_type: SessionType, duration_secs: u64) {
    let now = Local::now();
    let started_at = now - chrono::Duration::seconds(duration_secs as i64);
    log_session(&Session {
        session_type,
        duration_secs,
        started_at,
        completed_at: now,
    });
}

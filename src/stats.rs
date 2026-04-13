use crate::logger::read_sessions;
use crate::types::SessionType;
use chrono::{Datelike, Local};

/// Print a summary of session history.
pub fn show_stats() {
    let sessions = read_sessions();
    if sessions.is_empty() {
        println!("  No sessions logged yet. Start a Pomodoro!");
        return;
    }

    let today = Local::now().date_naive();
    let week_start = today - chrono::Duration::days(today.weekday().num_days_from_monday() as i64);

    let mut today_work = 0u64;
    let mut today_count = 0u32;
    let mut week_work = 0u64;
    let mut week_count = 0u32;
    let mut total_work = 0u64;
    let mut total_count = 0u32;

    for s in &sessions {
        if s.session_type != SessionType::Work {
            continue;
        }
        let date = s.started_at.date_naive();
        total_work += s.duration_secs;
        total_count += 1;

        if date >= week_start {
            week_work += s.duration_secs;
            week_count += 1;
        }
        if date == today {
            today_work += s.duration_secs;
            today_count += 1;
        }
    }

    println!();
    println!("  📊 Pomodoro Stats");
    println!("  ─────────────────────────────");
    println!(
        "  Today:      {} sessions, {} focused",
        today_count,
        format_duration(today_work)
    );
    println!(
        "  This week:  {} sessions, {} focused",
        week_count,
        format_duration(week_work)
    );
    println!(
        "  All time:   {} sessions, {} focused",
        total_count,
        format_duration(total_work)
    );
    println!();
}

fn format_duration(secs: u64) -> String {
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

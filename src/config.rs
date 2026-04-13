use crate::types::Config;
use std::fs;
use std::path::PathBuf;

/// Return the app data directory: ~/.pomodoro-cli/
pub fn data_dir() -> PathBuf {
    let dir = dirs::home_dir()
        .expect("Could not determine home directory")
        .join(".pomodoro-cli");
    fs::create_dir_all(&dir).expect("Could not create data directory");
    dir
}

/// Load config from ~/.pomodoro-cli/config.toml, falling back to defaults.
pub fn load_config() -> Config {
    let path = data_dir().join("config.toml");
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    }
}

/// Merge CLI overrides into the loaded config.
pub fn merge_overrides(
    mut cfg: Config,
    work: Option<u64>,
    short_break: Option<u64>,
    long_break: Option<u64>,
    rounds: Option<u32>,
    no_notify: bool,
) -> Config {
    if let Some(v) = work {
        cfg.work_mins = v;
    }
    if let Some(v) = short_break {
        cfg.short_break_mins = v;
    }
    if let Some(v) = long_break {
        cfg.long_break_mins = v;
    }
    if let Some(v) = rounds {
        cfg.rounds = v;
    }
    if no_notify {
        cfg.notify = false;
    }
    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = Config::default();
        assert_eq!(cfg.work_mins, 25);
        assert_eq!(cfg.short_break_mins, 5);
        assert_eq!(cfg.long_break_mins, 15);
        assert_eq!(cfg.rounds, 4);
        assert!(cfg.notify);
    }

    #[test]
    fn test_merge_overrides() {
        let cfg = Config::default();
        let merged = merge_overrides(cfg, Some(30), None, None, Some(6), true);
        assert_eq!(merged.work_mins, 30);
        assert_eq!(merged.short_break_mins, 5);
        assert_eq!(merged.rounds, 6);
        assert!(!merged.notify);
    }
}

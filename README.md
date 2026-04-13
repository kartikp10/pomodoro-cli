# 🍅 pomodoro-cli

A fast, minimal Pomodoro timer for the command line — written in Rust.

Runs entirely in your terminal with a live countdown, progress bar, desktop notifications, and session tracking.

## Install

```bash
# Build from source
git clone https://github.com/youruser/pomodoro-cli.git
cd pomodoro-cli
cargo install --path .
```

Requires Rust 1.70+ ([install](https://rustup.rs)).

## Usage

```bash
# Start a pomodoro session (25 min work → 5 min break → ... → 15 min long break)
pomodoro

# Same as above
pomodoro start

# Custom durations
pomodoro --work 50 --short-break 10 --long-break 20

# Fewer rounds before a long break
pomodoro --rounds 3

# Disable desktop notifications
pomodoro --no-notify

# View session stats
pomodoro stats

# Show current config
pomodoro config

# Clear session history
pomodoro reset
```

## Keyboard Controls

While the timer is running:

| Key         | Action       |
|-------------|--------------|
| `p` / Space | Pause/Resume |
| `s`         | Skip session |
| `q` / Esc   | Quit         |

## Session Cycle

```
Work (25m) → Short Break (5m) → Work → Short Break → ... → Long Break (15m)
             └──────── repeats for 4 rounds ────────┘
```

All durations and round count are configurable via CLI flags or `~/.pomodoro-cli/config.toml`:

```toml
work_mins = 25
short_break_mins = 5
long_break_mins = 15
rounds = 4
notify = true
```

## Data

- **Config**: `~/.pomodoro-cli/config.toml`
- **Session log**: `~/.pomodoro-cli/log.json`

## Commands

| Command           | Description                        |
|-------------------|------------------------------------|
| `pomodoro`        | Start a pomodoro session (default) |
| `pomodoro start`  | Same as above                      |
| `pomodoro stats`  | Show today/week/all-time stats     |
| `pomodoro config` | Print current configuration        |
| `pomodoro reset`  | Clear session history              |

## CLI Flags

| Flag              | Description                          | Default |
|-------------------|--------------------------------------|---------|
| `--work <min>`    | Work session duration in minutes     | 25      |
| `--short-break <min>` | Short break duration in minutes | 5       |
| `--long-break <min>`  | Long break duration in minutes  | 15      |
| `--rounds <n>`    | Work rounds before a long break      | 4       |
| `--no-notify`     | Disable desktop notifications        | false   |

## License

MIT

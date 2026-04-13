# 🍅 pomodoro-cli

A fast, minimal Pomodoro timer for the command line — written in Rust.

Runs entirely in your terminal with a live countdown, progress bar, desktop notifications, and session tracking.

## Install

### Prebuilt binaries

Download the latest release for your platform from [GitHub Releases](https://github.com/youruser/pomodoro-cli/releases):

```bash
# macOS (Apple Silicon)
curl -L https://github.com/youruser/pomodoro-cli/releases/latest/download/pomodoro-aarch64-apple-darwin.tar.gz | tar xz
sudo mv pomodoro /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/youruser/pomodoro-cli/releases/latest/download/pomodoro-x86_64-apple-darwin.tar.gz | tar xz
sudo mv pomodoro /usr/local/bin/

# Linux (x86_64)
curl -L https://github.com/youruser/pomodoro-cli/releases/latest/download/pomodoro-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv pomodoro /usr/local/bin/

# Linux (aarch64)
curl -L https://github.com/youruser/pomodoro-cli/releases/latest/download/pomodoro-aarch64-unknown-linux-gnu.tar.gz | tar xz
sudo mv pomodoro /usr/local/bin/
```

### From source

```bash
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

# Keep the screen awake while the timer runs
pomodoro --keep-awake

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
| `--keep-awake`    | Prevent display sleep while running   | false   |

## Releasing

Versions follow [Semantic Versioning](https://semver.org/). To cut a new release:

```bash
# Bump version in Cargo.toml, commit, and tag
./scripts/release.sh 0.2.0
git add Cargo.toml Cargo.lock
git commit -m "release: v0.2.0"
git tag v0.2.0
git push origin main --tags
```

Pushing a `v*.*.*` tag triggers the [release workflow](.github/workflows/release.yml) which:
1. Runs tests
2. Verifies the tag matches `Cargo.toml` version
3. Builds binaries for Linux (x86_64, aarch64), macOS (x86_64, aarch64), and Windows (x86_64)
4. Creates a GitHub Release with all artifacts and SHA-256 checksums

## License

[MIT](LICENSE.md)

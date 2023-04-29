# Pomo

Simple pomodoro timer CLI written in Rust.

## Installation

```bash
cargo install --git https://github.com/mskelton/pomo-rs
```

## Usage

### Get status

Prints the status of the current session.

```bash
pomo
```

### Start focus

Starts a new pomodoro focus session with the default duration.

```bash
pomo start
```

Or customize the session duration by providing a
[human like duration](https://github.com/tailhook/humantime).

```bash
pomo start 15m
```

### Start break

Starts a new break with the default duration.

```bash
pomo break
```

Or customize the break duration by providing a 
[human like duration](https://github.com/tailhook/humantime).

```bash
pomo break 10m
```

### Change duration

Changes the duration of the active session using the specified 
[human like duration](https://github.com/tailhook/humantime).

```bash
pomo duration 20m
```

### Stop session

Stops the current pomodoro session.

```bash
pomo stop
```

## Config

The default values for all commands can be customized by creating a `$HOME/.config/pomo/config.json` file. Below is an example of all available configuration options with their default values.

```json
{
  "durations": {
    "break": "5m",
    "focus": "30m"
  },
  "emojis": {
    "break": "🥂",
    "focus": "🍅",
    "warn": ["🔴", "⭕"]
  },
  "sound": "default"
}
```

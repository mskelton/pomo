# Pomo

Simple pomodoro timer CLI written in Rust.

## Installation

You can install Pomo by running the install script which will download
the [latest release](https://github.com/mskelton/pomo/releases/latest).

```bash
curl -LSfs https://mskelton.dev/pomo/install | sh
```

Or you can build from source.

```bash
git clone git@github.com:mskelton/pomo.git
cd pomo
cargo install --path .
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

### Toggle

If currently in a focus session, switches to a break, otherwise switch to a new
focus session. This uses the default session/break duration.

```bash
pomo toggle
```

Or customize the session/break duration by providing a 
[human like duration](https://github.com/tailhook/humantime).

```bash
pomo toggle 10m
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

### Working Hours

You can optional configure working hours for Pomo which will auto start/stop
when you start and end your working day.

```json
{
  "working_hours": {
    "start": "8am",
    "end": "5:30pm"
  }
}
```

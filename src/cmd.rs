use std::process;

use chrono::prelude::*;
use chrono::Duration;

use crate::config::read_config;
use crate::config::Config;
use crate::notifications::send_notification;
use crate::status::read_status;
use crate::status::{clear_status, write_status, Status, StatusType};

pub fn start_break(duration: &Option<String>, notify: bool) {
    let config = read_config();
    let duration_str = duration
        .to_owned()
        .unwrap_or(config.durations.break_duration)
        .to_string();

    let parsed_duration = match humantime::parse_duration(&duration_str) {
        Ok(d) => Duration::from_std(d).unwrap(),
        Err(_) => Duration::minutes(5),
    };

    write_status(Status {
        status_type: StatusType::Break,
        end: Utc::now() + parsed_duration,
        notified: false,
    });

    if notify {
        send_notification(
            String::from("Your break has started!"),
            config.emojis.break_emoji,
            config.sound,
        )
    }
}

pub fn change_duration(duration: &Option<String>) {
    let status = read_status();

    if status.is_none() || status.as_ref().unwrap().end < Utc::now() {
        println!("No session in progress");
        process::exit(1);
    }

    let mut status = status.unwrap();
    if duration.is_none() {
        return;
    }

    let parsed = humantime::parse_duration(duration.as_ref().unwrap());
    if parsed.is_err() {
        return;
    }

    status.end = Utc::now() + Duration::from_std(parsed.unwrap()).unwrap();
    write_status(status);
}

pub fn start_focus(duration: &Option<String>, notify: bool) {
    let config = read_config();
    let duration_str = duration
        .to_owned()
        .unwrap_or(config.durations.focus_duration)
        .to_string();

    let parsed_duration = match humantime::parse_duration(&duration_str) {
        Ok(d) => Duration::from_std(d).unwrap(),
        Err(_) => Duration::minutes(25),
    };

    write_status(Status {
        status_type: StatusType::Focus,
        end: Utc::now() + parsed_duration,
        notified: false,
    });

    if notify {
        send_notification(
            String::from("Your focus session has started!"),
            config.emojis.focus_emoji,
            config.sound,
        )
    }
}

pub fn stop_session(notify: bool) {
    clear_status();

    if notify {
        let config = read_config();

        send_notification(
            String::from("Your session has stopped!"),
            config.emojis.focus_emoji,
            config.sound,
        )
    }
}

pub fn print_status(no_emoji: bool) {
    let status = read_status();

    // Don't print anything if there is no active session
    if status.is_none() {
        return;
    }

    // Save IO ops by reading the config only if there is a running session
    let config = read_config();

    // Print the remaining time
    let mut status = status.unwrap();
    let remaining = status.end - Utc::now();
    let formatted = format_time(remaining);

    if no_emoji {
        print!("{}", formatted)
    } else {
        let emoji = get_emoji(&config, &status, remaining);

        print!("{} {}\n", emoji, formatted)
    }

    // Notify the user when the remaining time has elapsed
    if !status.notified && remaining.num_seconds() <= 0 {
        match status.status_type {
            StatusType::Focus => send_notification(
                String::from("Focus completed, let's take a break!"),
                config.emojis.break_emoji,
                config.sound,
            ),
            StatusType::Break => send_notification(
                String::from("Break is over, back to work!"),
                config.emojis.focus_emoji,
                config.sound,
            ),
        }

        // Update the status to indicate the notification has been queued to
        // prevent duplicate notifications.
        status.notified = true;
        write_status(status);
    }
}

/// Display the format either as 1h10m, 1m10s, or 10s based on the remaining
/// duration
fn format_time(duration: Duration) -> String {
    let seconds = duration.num_seconds() % 60;
    let minutes = (duration.num_seconds() / 60) % 60;
    let hours = (duration.num_seconds() / 60) / 60;
    let sign = if duration.num_seconds() < 0 { "-" } else { "" };

    if duration.num_hours().abs() >= 1 {
        format!("{}{}h{:02}m", sign, hours.abs(), minutes.abs())
    } else if duration.num_minutes().abs() >= 1 {
        format!("{}{}m{:02}s", sign, minutes.abs(), seconds.abs())
    } else {
        format!("{}{}s", sign, seconds.abs())
    }
}

fn get_emoji(config: &Config, status: &Status, remaining: Duration) -> String {
    // Cycle through the warning emojis to make the timer "blink" when used in a
    // statusline. This can be disabled by overriding the configuration to only
    // provide a single emoji.
    if remaining.num_seconds() <= 0 {
        let index = remaining.num_seconds().abs() as usize
            % config.emojis.warn_emoji.len();

        return match config.emojis.warn_emoji.get(index) {
            Some(emoji) => emoji.to_string(),
            None => String::from(""),
        };
    }

    match status.status_type {
        StatusType::Focus => config.emojis.focus_emoji.to_owned(),
        StatusType::Break => config.emojis.break_emoji.to_owned(),
    }
}

use std::process;

use chrono::prelude::*;
use chrono::Duration;

use crate::config::read_config;
use crate::config::Emojis;
use crate::notifications::send_notification;
use crate::status::{
    clear_status, read_status, write_status, Status, StatusType,
};
use crate::time;

pub fn start_break(duration: Option<String>, notify: bool, one_shot: bool) {
    let config = read_config();
    let duration_str =
        duration.unwrap_or(config.durations.break_duration).to_string();

    let parsed_duration = match humantime::parse_duration(&duration_str) {
        Ok(d) => Duration::from_std(d).unwrap(),
        Err(_) => Duration::try_minutes(5).unwrap(),
    };

    write_status(&Status {
        status_type: StatusType::Break,
        start: Utc::now(),
        end: Utc::now() + parsed_duration,
        last_notified: None,
        one_shot,
    });

    if notify {
        send_notification(
            String::from("Your break has started!"),
            &config.emojis.break_emoji,
            &config.sound.start,
        )
    }
}

pub fn change_duration(duration: Option<String>) {
    let status = read_status().filter(|s| s.end > Utc::now());

    if let Some(mut s) = status {
        let duration = &duration.unwrap_or_default();
        let parsed = humantime::parse_duration(duration);

        if let Ok(d) = parsed {
            s.end = Utc::now() + Duration::from_std(d).unwrap();
            write_status(&s);
        } else {
            println!("Invalid duration");
            process::exit(1);
        }
    } else {
        println!("No session in progress");
        process::exit(1);
    }
}

pub fn start_focus(duration: Option<String>, notify: bool, one_shot: bool) {
    let config = read_config();
    let duration_str =
        duration.unwrap_or(config.durations.focus_duration).to_string();

    let parsed_duration = match humantime::parse_duration(&duration_str) {
        Ok(d) => Duration::from_std(d).unwrap(),
        Err(_) => Duration::try_minutes(25).unwrap(),
    };

    write_status(&Status {
        status_type: StatusType::Focus,
        start: Utc::now(),
        end: Utc::now() + parsed_duration,
        last_notified: None,
        one_shot,
    });

    if notify {
        send_notification(
            String::from("Your focus session has started!"),
            &config.emojis.focus_emoji,
            &config.sound.start,
        )
    }
}

pub fn toggle_session(duration: Option<String>, notify: bool, one_shot: bool) {
    let status = read_status();

    // If there is an active session, start a break, otherwise start a new
    // focus session.
    match status.map(|s| s.status_type) {
        Some(StatusType::Focus) => start_break(duration, notify, one_shot),
        _ => start_focus(duration, notify, one_shot),
    }
}

pub fn stop_session(notify: bool) {
    clear_status();

    if notify {
        let config = read_config();

        send_notification(
            String::from("Your session has stopped!"),
            &config.emojis.focus_emoji,
            &config.sound.end,
        )
    }
}

pub fn print_status(no_emoji: bool, notify: bool) -> Option<()> {
    let mut status = read_status()?;
    let config = read_config();
    let work_start = time::parse_time(config.working_hours.start);
    let work_end = time::parse_time(config.working_hours.end);

    // If the end of last session was before the start of the work day, and
    // the work day has started, then start a new focus session.
    if let Some(start) = work_start {
        if status.end < start && Utc::now() > start {
            start_focus(None, false, false);
            return Some(());
        }
    }

    // If the current time was started before the end of the work day, and now
    // the work day has ended, then clear the status.
    if let Some(end) = work_end {
        if status.start < end && Utc::now() > end {
            clear_status();
            return Some(());
        }
    }

    // Bail if there is no active session
    if status.status_type == StatusType::Idle {
        return None;
    }

    // Print the remaining time
    let remaining = status.end - Utc::now();
    let formatted = format_time(remaining);

    // Notify the user when the remaining time has elapsed. After that, notify
    // the user every 5 minutes to remind them to take a break.
    if notify && remaining.num_seconds() <= 0 && should_notify(&status) {
        match status.status_type {
            StatusType::Focus => send_notification(
                String::from("Focus completed, let's take a break!"),
                &config.emojis.break_emoji,
                &config.sound.end,
            ),
            StatusType::Break => send_notification(
                String::from("Break is over, back to work!"),
                &config.emojis.focus_emoji,
                &config.sound.end,
            ),
            StatusType::Idle => (),
        }

        // Update the status to indicate the notification has been queued to
        // prevent duplicate notifications.
        status.last_notified = Some(Utc::now());
        write_status(&status);
    }

    // If the one-shot flag is set and the remaining time has elapsed, then
    // clear the status and exit without printing.
    if status.one_shot && remaining.num_seconds() <= 0 {
        clear_status();
        return Some(());
    }

    if no_emoji {
        print!("{}", formatted)
    } else {
        let emoji = get_emoji(&config.emojis, &status, remaining);
        print!("{} {}\n", emoji, formatted)
    }

    Some(())
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
        format!("{}{}m", sign, minutes.abs())
    } else {
        format!("{}{}s", sign, seconds.abs())
    }
}

fn get_emoji(emojis: &Emojis, status: &Status, remaining: Duration) -> String {
    // Cycle through the warning emojis to make the timer "blink" when used in a
    // statusline. This can be disabled by overriding the configuration to only
    // provide a single emoji.
    if remaining.num_seconds() <= 0 {
        let index =
            remaining.num_seconds().abs() as usize % emojis.warn_emoji.len();

        return match emojis.warn_emoji.get(index) {
            Some(emoji) => emoji.to_string(),
            None => String::from(""),
        };
    }

    match status.status_type {
        StatusType::Focus => emojis.focus_emoji.to_owned(),
        StatusType::Break => emojis.break_emoji.to_owned(),
        StatusType::Idle => "".to_owned(),
    }
}

/// Determine if the user should be notified based on the last time they were
/// notified.
fn should_notify(status: &Status) -> bool {
    status
        .last_notified
        .map_or(true, |t| t < (Utc::now() - Duration::try_minutes(5).unwrap()))
}

use std::process::Command;

pub fn send_notification(message: String, emoji: String, sound: String) {
    Command::new("osascript")
        .arg("-e")
        .arg(format!(
            "tell application \"System Events\" to display notification \
             \"{}\" with title \"Pomo {}\" sound name \"{}\"",
            message, emoji, sound
        ))
        .spawn()
        .expect("failed to send notification");
}

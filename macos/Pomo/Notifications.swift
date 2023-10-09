import Foundation
import UserNotifications

func maybeNotify(status: Status, config: Config) {
    // Notify the user when the remaining time has elapsed. After that, notify
    // the user every 5 minutes to remind them to take a break.
    let remaining = status.end.timeIntervalSinceNow

    if remaining <= 0, shouldNotify(status: status) {
        switch status.type {
        case .Focus:
            sendNotification(
                "Focus completed, let's take a break!",
                emoji: config.emojis?.breakEmoji ?? "🥛",
                sound: UNNotificationSoundName(rawValue: "Glass")
//                emoji: config.emojis.breakEmoji,
//                sound: config.sound.end
            )

        case .Break:
            sendNotification(
                "Break is over, back to work!",
                emoji: config.emojis?.breakEmoji ?? "🥛",
                sound: UNNotificationSoundName(rawValue: "Glass")
//                emoji: config.emojis.focusEmoji,
//                sound: config.sound.end
            )

        case .Idle:
            return
        }

        // Update the status to indicate the notification has been queued to
        // prevent duplicate notifications.
        writeStatus(Status(
            type: status.type,
            start: status.start,
            end: status.end,
            lastNotified: Date()
        ))
    }
}

func shouldNotify(status: Status) -> Bool {
    guard let lastNotified = status.lastNotified else {
        return true
    }

    return lastNotified.timeIntervalSinceNow < -300
}

func sendNotification(_ text: String, emoji: String, sound: UNNotificationSoundName) {
    UNUserNotificationCenter.current().requestAuthorization(options: [.alert, .sound]) { success, _ in
        if success {
            let content = UNMutableNotificationContent()
            content.title = "Pomo \(emoji)"
            content.subtitle = text
            content.sound = UNNotificationSound(named: sound)

            let request = UNNotificationRequest(identifier: UUID().uuidString, content: content, trigger: nil)
            UNUserNotificationCenter.current().add(request)
        }
    }
}

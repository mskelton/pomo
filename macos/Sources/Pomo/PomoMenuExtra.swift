import SwiftUI

struct PomoMenuExtra: Scene {
    @State var text = update(status: nil)

    let timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()

    var body: some Scene {
        MenuBarExtra {
            Button("Start focus session", action: startFocus).keyboardShortcut("f")
            Button("Start break", action: startBreak).keyboardShortcut("b")
            Button("Stop session", action: stopSession).keyboardShortcut("s")
            Divider()
            Button("Quit", action: quit).keyboardShortcut("q")
        } label: {
            if text == "" {
                Image(systemName: "timer")
            } else {
                Text(text).onReceive(timer) { _ in
                    text = update(status: nil)
                }
            }
        }
    }

    func startFocus() {
        text = update(status: Status(
            type: .Focus,
            start: Date(),
            end: Date().addingTimeInterval(30 * 60),
            lastNotified: nil
        ))
    }

    func startBreak() {
        text = update(status: Status(
            type: .Break,
            start: Date(),
            end: Date().addingTimeInterval(5 * 60),
            lastNotified: nil
        ))
    }

    func stopSession() {
        text = update(status: Status(
            type: .Idle,
            start: Date(),
            end: Date(),
            lastNotified: nil
        ))
    }

    func quit() {
        NSApplication.shared.terminate(nil)
    }
}

func update(status: Status?) -> String {
    if let status = status {
        writeStatus(status)
    }

    let status = status ?? getStatus()

    // Update the title with the current session time
    return status.type == StatusType.Idle
        ? ""
        : formatDuration(status.end.timeIntervalSinceNow)
}

func formatDuration(_ duration: TimeInterval) -> String {
    let hours = abs(Int(duration) / 3600)
    let minutes = abs(Int(duration / 60) % 60)
    let seconds = abs(Int(duration) % 60)
    let sign = duration < 0 ? "-" : ""

    if hours >= 1 {
        return String(format: "%@%dh%02dm", sign, hours, minutes)
    } else if minutes >= 1 {
        return String(format: "%@%dm%02ds", sign, minutes, seconds)
    } else {
        return String(format: "%@%2ds", sign, seconds)
    }
}

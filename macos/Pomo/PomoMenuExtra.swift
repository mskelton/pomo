import SwiftUI
import UserNotifications

struct PomoMenuExtra: Scene {
    @State var config = getConfig()
    @State var text = update(config: getConfig(), status: nil)

    let timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()
    let update_config = Timer.publish(every: 600, on: .main, in: .common).autoconnect()

    var body: some Scene {
        MenuBarExtra {
            Button("Start focus session", action: startFocus).keyboardShortcut("f")
            Button("Start break", action: startBreak).keyboardShortcut("b")
            Button("Stop session", action: stopSession).keyboardShortcut("s")
            Divider()
            Button("Quit", action: quit).keyboardShortcut("q")
        } label: {
            Text(text.isEmpty ? "" : text)
                .onReceive(timer) { _ in
                    text = update(config: config, status: nil)
                }
                .onReceive(update_config) { _ in
                    config = getConfig()
                }
                .overlay(text.isEmpty ? Image(systemName: "timer") : nil)
        }
    }

    func startFocus() {
        text = update(config: config, status: Status(
            type: .Focus,
            start: Date(),
            end: Date().addingTimeInterval(30 * 60),
            lastNotified: nil,
            oneShot: false
        ))
    }

    func startBreak() {
        text = update(config: config, status: Status(
            type: .Break,
            start: Date(),
            end: Date().addingTimeInterval(5 * 60),
            lastNotified: nil,
            oneShot: false
        ))
    }

    func stopSession() {
        text = update(config: config, status: Status(
            type: .Idle,
            start: Date(),
            end: Date(),
            lastNotified: nil,
            oneShot: false
        ))
    }

    func quit() {
        NSApplication.shared.terminate(nil)
    }
}

func update(config: Config, status: Status?) -> String {
    if let status = status {
        writeStatus(status)
    }

    let status = status ?? getStatus()
    maybeNotify(status: status, config: config)

    // Update the title with the current session time
    return status.type == StatusType.Idle
        ? ""
        : formatDuration(status.end.timeIntervalSinceNow)
}

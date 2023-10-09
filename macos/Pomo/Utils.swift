import Foundation

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

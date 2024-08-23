import Foundation

enum StatusType: String, Codable {
    case Idle, Focus, Break
}

struct Status: Codable {
    let type: StatusType
    let start: Date
    let end: Date
    let lastNotified: Date?
    let oneShot: Bool
}

func getStatus() -> Status {
    do {
        let data = try Data(contentsOf: getStatusURL())
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .formatted(getDateFormatter())
        decoder.keyDecodingStrategy = .convertFromSnakeCase

        let status = try decoder.decode(Status.self, from: data)
        return status
    } catch {
        return Status(
            type: .Idle,
            start: Date(),
            end: Date(),
            lastNotified: Date(),
            oneShot: false
        )
    }
}

func writeStatus(_ status: Status) {
    do {
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .formatted(getDateFormatter())
        encoder.keyEncodingStrategy = .convertToSnakeCase

        let data = try encoder.encode(status)
        try data.write(to: getStatusURL())
    } catch {
        print("error: failed to write status")
    }
}

func getStatusURL() -> URL {
    let home = NSHomeDirectory()
    let filePath = "\(home)/.config/pomo/status.json"
    return URL(fileURLWithPath: filePath)
}

func getDateFormatter() -> DateFormatter {
    let formatter = DateFormatter()

    formatter.locale = Locale(identifier: "en_US_POSIX")
    formatter.dateFormat = "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"
    formatter.timeZone = TimeZone(secondsFromGMT: 0)

    return formatter
}

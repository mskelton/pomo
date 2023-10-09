import Foundation

struct Durations: Codable {
    let breakDuration: String?
    let focusDuration: String?

    enum CodingKeys: String, CodingKey {
        case breakDuration = "break"
        case focusDuration = "focus"
    }
}

struct Emojis: Codable {
    let breakEmoji: String?
    let focusEmoji: String?
    let warnEmoji: [String]?

    enum CodingKeys: String, CodingKey {
        case breakEmoji = "break"
        case focusEmoji = "focus"
        case warnEmoji = "warn"
    }
}

struct Sounds: Codable {
    let start: String?
    let end: String?
}

struct WorkingHours: Codable {
    let start: String?
    let end: String?
}

struct Config: Codable {
    let durations: Durations?
    let emojis: Emojis?
    let sound: Sounds?
    let workingHours: WorkingHours?
}

func getConfig() -> Config {
    do {
        let data = try Data(contentsOf: getConfigURL())
        let decoder = JSONDecoder()
        decoder.keyDecodingStrategy = .convertFromSnakeCase

        let status = try decoder.decode(Config.self, from: data)
        return status
    } catch {
        return Config(
            durations: nil,
            emojis: nil,
            sound: nil,
            workingHours: nil
        )
    }
}

func getConfigURL() -> URL {
    let home = NSHomeDirectory()
    let filePath = "\(home)/.config/pomo/config.json"
    return URL(fileURLWithPath: filePath)
}

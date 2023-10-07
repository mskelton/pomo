// swift-tools-version:5.7
import PackageDescription

let package = Package(
    name: "Pomo",
    platforms: [.macOS(.v13)],
    products: [
        .application(
            name: "Pomo",
            targets: ["Pomo"]
        ),
    ],
    dependencies: [],
    targets: [
        .target(
            name: "Pomo",
            resources: [.process("Resources")]
        ),
    ]
)

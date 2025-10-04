// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "ProveItSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16)
    ],
    products: [
        .library(
            name: "ProveItSwift",
            targets: ["ProveItSwift"]),
    ],
    targets: [
        .target(
            name: "ProveItSwift",
            dependencies: []),
        .testTarget(
            name: "ProveItSwiftTests",
            dependencies: ["ProveItSwift"]),
    ]
)

// swift-tools-version: 5.10
import PackageDescription

let package = Package(
    name: "Keystone",
    platforms: [.macOS(.v14)],
    products: [
        .library(name: "KeystoneUI", targets: ["KeystoneUI"]),
    ],
    dependencies: [
        // 数式レンダリング（数学・物理問題の表示）
    ],
    targets: [
        .target(
            name: "KeystoneUI",
            path: "Sources/UI"
        ),
        // TODO: Bridge ターゲット（Rust FFI ヘッダを取り込む）
    ]
)

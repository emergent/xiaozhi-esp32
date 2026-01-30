# XiaoZhi Rust Component

This component provides Rust implementations of XiaoZhi features for **M5Stack Atom EchoS3R (ESP32-S3)**, gradually migrating from C++ while maintaining compatibility through FFI.

## Target Hardware

- **Device**: M5Stack Atom EchoS3R
- **Chip**: ESP32-S3 (Xtensa architecture)
- **Flash**: 8MB
- **Architecture**: Xtensa LX7 dual-core

## Structure

```
components/xiaozhi-rust/
├── Cargo.toml           # Rust package manifest
├── build.rs             # Build script for ESP-IDF integration
├── CMakeLists.txt       # ESP-IDF component integration
├── rust-toolchain.toml  # Rust toolchain specification (ESP Xtensa)
├── .cargo/
│   └── config.toml      # Cargo build configuration
├── include/
│   └── xiaozhi_rust.h   # C/C++ header for FFI
└── src/
    └── lib.rs           # Main Rust library
```

## Setup

### 1. Install ESP Rust Toolchain (Xtensa)

ESP32-S3はXtensaアーキテクチャのため、専用のESPツールチェーンが必要です。

```bash
# Install espup (ESP Rust toolchain installer)
cargo install espup

# Install ESP Rust toolchain with Xtensa support
espup install

# Source the environment variables (必須)
# Linux/macOS:
source $HOME/export-esp.sh
# Windows:
# .\export-esp.ps1

# シェルの起動時に自動的に読み込むには、以下を~/.bashrc or ~/.zshrcに追加:
# source $HOME/export-esp.sh
```

### 2. Install Additional Tools

```bash
# Install ldproxy (リンカープロキシ)
cargo install ldproxy

# Install cargo-espflash (フラッシュツール)
cargo install cargo-espflash espflash
```

### 3. Verify Installation

```bash
# Xtensaターゲットが利用可能か確認
rustup toolchain list | grep esp
# 出力例: esp (default)

# ターゲットを確認
rustup target list --installed --toolchain esp | grep xtensa
# 出力例: xtensa-esp32s3-espidf
```

## Building

### Standalone Rust Build

```bash
cd components/xiaozhi-rust

# ビルド (ESP32-S3向け)
cargo build --release

# ターゲットを明示的に指定する場合
cargo build --release --target xtensa-esp32s3-espidf
```

### Integrated with ESP-IDF

Rustコンポーネントは、メインプロジェクトのビルド時に自動的にビルドされます：

```bash
cd ../..  # プロジェクトルートに戻る

# ボードタイプをM5Stack Atom EchoS3Rに設定
idf.py set-target esp32s3
idf.py menuconfig
# Board Configuration -> Board Type -> M5Stack Atom EchoS3R

# ビルド
idf.py build

# フラッシュとモニター
idf.py flash monitor
```

## Testing

```bash
cd components/xiaozhi-rust

# ユニットテスト (ホスト環境)
cargo test

# Lintチェック
cargo clippy -- -D warnings

# フォーマット
cargo fmt

# フォーマットチェック
cargo fmt -- --check
```

## FFI Integration

### C++ から Rust を呼び出す

main.ccにRustの初期化を追加：

```cpp
#include "xiaozhi_rust.h"

extern "C" void app_main(void)
{
    // 既存の初期化...

    // Rustランタイムの初期化
    xiaozhi_rust_init();

    // バージョン確認
    const char* rust_version = xiaozhi_rust_version();
    ESP_LOGI("main", "Rust component version: %s", rust_version);

    // 既存のアプリケーション起動...
}
```

### Rust から ESP-IDF を呼び出す

```rust
use esp_idf_svc::sys as esp_idf_sys;
use esp_idf_svc::hal::gpio::*;

// ESP-IDF APIの呼び出し例
pub fn example_gpio() -> Result<(), EspError> {
    let peripherals = Peripherals::take()?;
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    led.set_high()?;
    Ok(())
}
```

## Development Guidelines

### Safety

- `unsafe`ブロックは最小限に
- すべての`unsafe`コードに安全性の要件を文書化
- C APIは安全なRust抽象化でラップ

### Error Handling

- すべてのエラー可能な操作に`Result<T, E>`を使用
- `thiserror`でカスタムエラー型を定義
- 本番コードでは`panic!`を避ける

### Memory Management

- ESP32-S3のスタックサイズに注意（デフォルト8KB）
- 大きな構造体はヒープに配置
- 定期的にメモリ使用量をプロファイル

### Performance

- `release`プロファイルで最適化（`opt-level = "z"`）
- LTOを有効化してコードサイズを削減
- クリティカルパスはベンチマーク

## Target Specifications

- **Target**: `xtensa-esp32s3-espidf`
- **Architecture**: Xtensa LX7
- **Cores**: 2
- **Clock**: Up to 240MHz
- **SRAM**: 512KB
- **Flash**: 8MB (M5Stack Atom EchoS3R)

## Resources

- [The Rust on ESP Book](https://esp-rs.github.io/book/)
- [esp-idf-svc Documentation](https://docs.rs/esp-idf-svc/)
- [ESP-RS GitHub](https://github.com/esp-rs)
- [M5Stack Atom EchoS3R](https://docs.m5stack.com/en/atom/atom_echos3r)

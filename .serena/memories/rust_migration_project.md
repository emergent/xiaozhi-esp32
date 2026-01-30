# Rust移行プロジェクト

## プロジェクト概要

**ブランチ**: `rustize`

既存のC++ベースのXiaoZhi AI Chatbotを段階的にRustに移行するプロジェクト。完全な書き換えではなく、モジュール単位で安全に移行していく。

## 目標

1. Rustのメモリ安全性と型安全性を活用
2. 並行処理の改善（Rustの並行処理モデル + FreeRTOS）
3. 保守性の向上（明確なエラーハンドリング）
4. パフォーマンス維持または改善
5. C++コードとの共存

## 使用技術

### Rustエコシステム
- **esp-idf-sys**: ESP-IDF C APIへのFFIバインディング
- **esp-idf-svc**: ESP-IDFサービスのRustラッパー
- **esp-idf-hal**: ハードウェア抽象化レイヤー
- **embedded-svc**: 組み込みサービスの抽象化

### ビルドシステム
- Cargo + ESP-IDF CMake統合
- embuild（ESP-IDF対応ビルドスクリプト）

## 段階的移行戦略

### フェーズ1: 基盤構築（現在）
- Cargoプロジェクトのセットアップ
- ESP-IDF依存関係の設定
- C++とRustの相互運用基盤（FFI）
- 最初の簡単なモジュール実装

### フェーズ2: 独立モジュール移行
優先順位：
1. 設定管理（settings）
2. システム情報（system_info）
3. LED制御など単純なドライバー

### フェーズ3: 中核機能移行
1. プロトコルハンドラー（WebSocket、MQTT）
2. 状態マシン
3. オーディオパイプライン

### フェーズ4: 統合と最適化
1. メモリ最適化
2. エラーハンドリング統一
3. パフォーマンスベンチマーク

## 移行の原則

1. **FFI境界の明確化**: C++とRustの境界を明確に定義
2. **安全性優先**: `unsafe`は最小限、必ず安全なラッパーで包む
3. **段階的置き換え**: 小さな単位でテスト可能に
4. **パフォーマンス検証**: 組み込み環境のリソース制約に注意

## Rustコーディング規約

### スタイル
- `rustfmt`デフォルト設定
- 型名: `PascalCase`
- 関数/変数: `snake_case`
- 定数: `SCREAMING_SNAKE_CASE`

### エラーハンドリング
- `Result<T, E>`を積極的に使用
- カスタムエラー型を定義
- `panic!`を避ける

### ドキュメント
- パブリックAPIに`///`コメント
- サンプルコード、エラーケースを明記

## 注意事項

### メモリ制約
- ESP32 SRAMは限られている（ESP32-S3で512KB）
- ヒープ使用量に注意
- 大きな構造体はヒープに配置

### リアルタイム性
- FreeRTOSタスク優先度を考慮
- Rustの`async`とFreeRTOSの統合に注意

### ビルドサイズ
- リリースビルドは最適化（`opt-level = "z"`）
- LTO有効化（`lto = true`）
- パニックハンドラー軽量化（`panic = "abort"`）

## 開発コマンド

### Rust toolchain
```bash
rustup target add xtensa-esp32s3-espidf  # ESP32-S3
cargo install cargo-espflash espflash
```

### ビルド
```bash
cargo build --release
idf.py build
```

### テスト
```bash
cargo test           # ユニットテスト
idf.py flash monitor # デバイステスト
```

### リント
```bash
cargo fmt --check
cargo clippy -- -D warnings
```

## 参考リソース
- [The Rust on ESP Book](https://esp-rs.github.io/book/)
- [esp-rs GitHub](https://github.com/esp-rs)
- [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/)

## 現在の進捗
- **フェーズ**: フェーズ1（基盤構築）
- **次のステップ**: Cargoプロジェクトのセットアップ

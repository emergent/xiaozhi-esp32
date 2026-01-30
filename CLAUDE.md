# XiaoZhi ESP32 - Rust移行プロジェクト

## プロジェクトコンテキスト

このブランチ（`rustize`）では、既存のC++ベースのXiaoZhi AI Chatbotを段階的にRustに移行するチャレンジを行います。完全な書き換えではなく、**段階的かつ安全な移行**を目指します。

### 現在の状態
- **メインブランチ**: `main` (C++実装)
- **開発ブランチ**: `rustize` (C++ → Rust移行中)
- **ベースフレームワーク**: ESP-IDF (Espressif IoT Development Framework)
- **対象チップ**: ESP32-C3, ESP32-C5, ESP32-C6, ESP32-S3, ESP32-P4

## Rust移行の目標

1. **安全性の向上**: Rustのメモリ安全性と型安全性を活用
2. **並行処理の改善**: Rustの並行処理モデルでFreeRTOSタスク管理を簡素化
3. **保守性の向上**: より明確なエラーハンドリングと型システム
4. **段階的移行**: 既存のC++コードと共存しながら、モジュール単位で移行
5. **パフォーマンス維持**: 組み込み環境でのパフォーマンスを維持または改善

## Rust + ESP-IDF統合

### 使用するエコシステム
- **[esp-idf-sys](https://github.com/esp-rs/esp-idf-sys)**: ESP-IDF C APIへのRustバインディング
- **[esp-idf-svc](https://github.com/esp-rs/esp-idf-svc)**: ESP-IDFサービスのRustラッパー
- **[esp-idf-hal](https://github.com/esp-rs/esp-idf-hal)**: ハードウェア抽象化レイヤー
- **[embedded-svc](https://github.com/esp-rs/embedded-svc)**: 組み込みサービスの抽象化トレイト

### ビルドシステム
- **標準的なアプローチ**: ESP-IDFのCMakeとCargoを統合
- **embuild**: ESP-IDF対応のビルドスクリプトサポート

## 段階的移行戦略

### フェーズ1: 基盤構築（現在のフェーズ）
- [ ] Cargoプロジェクトのセットアップ
- [ ] ESP-IDF依存関係の設定
- [ ] C++とRustの相互運用基盤の構築
- [ ] 最初の簡単なモジュール（例：ユーティリティ関数）のRust実装

### フェーズ2: 独立モジュールの移行
優先度順に以下のモジュールを移行：
1. **設定管理（settings.cc/h）**: 状態管理、シリアライゼーション
2. **システム情報（system_info.cc/h）**: システム情報の取得
3. **単純なドライバー**: LED制御など状態を持たないモジュール

### フェーズ3: 中核機能の移行
1. **プロトコルハンドラー（protocols/）**: WebSocket、MQTT
2. **状態マシン（device_state_machine.cc/h）**: Rustの型システムで状態を表現
3. **オーディオパイプライン**: ストリーム処理をRustの非同期機能で実装

### フェーズ4: 統合と最適化
1. **メモリ使用量の最適化**
2. **エラーハンドリングの統一**
3. **パフォーマンスベンチマーク**
4. **ドキュメント更新**

## 移行の原則

### 1. FFI（Foreign Function Interface）の活用
- C++とRustの境界を明確に定義
- `extern "C"` インターフェースで相互運用
- 型安全なラッパーを作成

### 2. 安全性優先
- `unsafe`ブロックは最小限に
- すべての`unsafe`コードに詳細なコメントを付与
- C APIを呼び出す際は、必ずRustの安全な抽象化でラップ

### 3. 段階的な置き換え
- 一度に大きなモジュールを書き換えない
- 小さな単位でテスト可能な形で移行
- 各ステップでビルドとテストが通ることを確認

### 4. パフォーマンス検証
- 組み込み環境では、スタックとヒープの使用量に注意
- ベンチマークを取り、C++版と比較
- `no_std`が必要な箇所は検討（ただし、ESP-IDFは`std`をサポート）

## コーディング規約（Rust）

### スタイル
- **標準スタイル**: `rustfmt`のデフォルト設定を使用
- **命名**: Rustの命名規則に従う
  - 型名: `PascalCase`
  - 関数/変数: `snake_case`
  - 定数: `SCREAMING_SNAKE_CASE`
  - トレイト: `PascalCase`

### ドキュメント
- パブリックAPIには必ずドキュメントコメント（`///`）を付与
- サンプルコードを含める（`/// # Examples`）
- エラーケースを明記（`/// # Errors`）

### エラーハンドリング
- `Result<T, E>`を積極的に使用
- カスタムエラー型を定義（`thiserror`クレートを検討）
- `panic!`は避け、`Result`で伝播

### 非同期処理
- ESP-IDFの`embassy`サポートを検討
- FreeRTOSとの統合にはスレッド同期が必要

## 推奨クレート（組み込み向け）

### コア
- `esp-idf-sys` - ESP-IDF FFIバインディング
- `esp-idf-svc` - ESP-IDFサービス
- `esp-idf-hal` - ハードウェア抽象化
- `embedded-hal` - 汎用ハードウェアトレイト

### ユーティリティ
- `log` - ログ出力（ESP-IDFのログシステムと統合）
- `heapless` - ヒープレスコレクション
- `serde` - シリアライゼーション（`no_std`対応）
- `thiserror` - エラー型定義

### 通信
- `embedded-svc` - ネットワークサービス抽象化

## 開発ワークフロー

### セットアップ

#### ESP-IDF環境変数の設定（毎回必要）
```bash
# ESP-IDF v5.5.2 のセットアップ
source ~/esp/v5.5.2/esp-idf/export.sh

# または、シェルの起動時に自動設定するには ~/.zshrc または ~/.bashrc に追加:
# alias idf='source ~/esp/v5.5.2/esp-idf/export.sh'
```

#### Rust環境のセットアップ（初回のみ）
```bash
# Rust toolchain のインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# ESP Rust toolchain のインストール
cargo install espup
espup install

# Rustツールのインストール
cargo install cargo-espflash espflash ldproxy

# Rust環境変数の設定（毎回必要）
source ~/export-esp.sh
```

### ビルド
```bash
# Rustコンポーネントのビルド
cargo build --release

# ESP-IDF統合ビルド
idf.py build
```

### テスト
```bash
# Rustユニットテスト（ホスト環境）
cargo test

# デバイステスト
idf.py flash monitor
```

### リント
```bash
# フォーマットチェック
cargo fmt --check

# リント
cargo clippy -- -D warnings
```

## 対話の進め方

### タスクの提案
プロジェクトの現状を踏まえて、次に移行すべきモジュールや実装すべき機能を提案してください。以下の観点を考慮：
1. **依存関係**: 他のモジュールへの依存が少ないものから
2. **複雑度**: シンプルなものから複雑なものへ
3. **価値**: Rust化によるメリットが大きいもの優先
4. **リスク**: 失敗しても影響が小さいものから

### 実装の進め方
各タスクについて：
1. **設計レビュー**: FFI境界、データ構造、エラーハンドリングを議論
2. **実装**: 段階的に実装、こまめにコミット
3. **テスト**: ユニットテスト + 実機テスト
4. **ドキュメント**: コードコメント + CLAUDE.mdの更新

### 質問と調査
- 不明点があれば積極的に質問
- ESP-IDF、Rustの組み込み開発のベストプラクティスを調査
- 既存のC++実装を理解してから移行を提案

## 注意事項

### ESP-IDFバージョン互換性
- ESP-IDF 5.4+ を対象
- `esp-idf-sys`のバージョンとESP-IDFバージョンの互換性を確認

### メモリ制約
- ESP32のRAMは限られている（ESP32-S3で512KB SRAM）
- Rustのstdライブラリは使用可能だが、ヒープ使用量に注意
- 大きな構造体はスタックではなくヒープに配置

### リアルタイム性
- FreeRTOSのタスク優先度を考慮
- Rustの`async`とFreeRTOSタスクの統合に注意

### ビルドサイズ
- リリースビルドは最適化（`opt-level = "z"`を検討）
- 未使用コードの削除（`lto = true`）
- パニックハンドラーを軽量化（`panic = "abort"`）

## 参考リソース

### 公式ドキュメント
- [The Rust on ESP Book](https://esp-rs.github.io/book/)
- [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)

### コミュニティ
- [esp-rs GitHub Organization](https://github.com/esp-rs)
- [ESP32 Rust Matrix Channel](https://matrix.to/#/#esp-rs:matrix.org)

## 進捗管理

### 現在の状態
- **フェーズ**: フェーズ1（基盤構築）
- **ターゲットデバイス**: M5Stack Atom EchoS3R (ESP32-S3, Xtensa)
- **次のステップ**: FFI基盤の構築とC++からの呼び出しテスト

### 完了したタスク

#### 2026-01-30: Cargoプロジェクトのセットアップ ✅
- [x] `components/xiaozhi-rust/` ディレクトリ作成
- [x] `Cargo.toml` 作成（ESP32-S3向け設定）
- [x] `build.rs` 作成（ESP-IDF統合）
- [x] `src/lib.rs` 作成（初期FFI関数実装）
- [x] `.cargo/config.toml` 作成（Xtensaターゲット設定）
- [x] `rust-toolchain.toml` 作成（ESPツールチェーン指定）
- [x] `include/xiaozhi_rust.h` 作成（C/C++ヘッダー）
- [x] `CMakeLists.txt` 作成（ESP-IDFビルドシステム統合）
- [x] Xtensaツールチェーンのインストール（`espup install`）
- [x] 依存関係の修正（esp-idf-svc 0.51 + alloc/std features）
- [x] リリースビルド成功（libxiaozhi_rust.a: 1.3MB）

#### 実装済みのFFI関数
- `xiaozhi_rust_init()`: Rustランタイムとロギングの初期化
- `xiaozhi_rust_version()`: バージョン文字列の取得

### 課題と疑問点
（対話の中で追記）

---

**このドキュメントは、プロジェクトの進行に合わせて随時更新します。**

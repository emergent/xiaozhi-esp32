# プロジェクト構造

## ディレクトリ構成

```
xiaozhi-esp32/
├── .github/              # GitHub Actions CI/CD設定
│   └── workflows/        # ワークフロー定義
├── docs/                 # ドキュメント
│   ├── custom-board.md   # カスタムボードガイド
│   ├── mcp-usage.md      # MCPプロトコル使用方法
│   ├── mcp-protocol.md   # MCPプロトコル詳細
│   ├── websocket.md      # WebSocket通信プロトコル
│   ├── mqtt-udp.md       # MQTT+UDP通信プロトコル
│   └── blufi.md          # BLUFI設定
├── main/                 # メインアプリケーションコード
│   ├── audio/            # オーディオ処理
│   │   ├── codecs/       # オーディオコーデックドライバー
│   │   ├── processors/   # オーディオプロセッサー
│   │   └── wake_words/   # ウェイクワード検出
│   ├── boards/           # ボード固有の設定（70+ボード）
│   │   ├── common/       # 共通ボードコード
│   │   ├── esp-box-3/    # ESP-BOX-3設定
│   │   ├── m5stack-core-s3/ # M5Stack CoreS3設定
│   │   └── ...           # その他のボード
│   ├── display/          # ディスプレイ関連
│   │   └── lvgl_display/ # LVGL統合
│   ├── led/              # LED制御
│   ├── protocols/        # 通信プロトコル
│   ├── assets/           # アセット（音声、画像、フォント）
│   │   ├── locales/      # 多言語対応ファイル
│   │   └── common/       # 共通アセット
│   ├── main.cc           # メインエントリーポイント
│   ├── application.cc/h  # アプリケーションロジック
│   ├── device_state_machine.cc/h # デバイス状態マシン
│   ├── mcp_server.cc/h   # MCPサーバー実装
│   ├── ota.cc/h          # OTAアップデート
│   ├── settings.cc/h     # 設定管理
│   └── CMakeLists.txt    # コンポーネントビルド設定
├── managed_components/   # ESP-IDF管理コンポーネント
├── partitions/           # パーティションテーブル定義
│   └── v2/               # v2パーティションテーブル
├── scripts/              # ビルド/デプロイスクリプト
│   ├── release.py        # リリースビルドスクリプト
│   ├── gen_lang.py       # 言語設定生成
│   ├── build_default_assets.py # アセットビルド
│   └── mp3_to_ogg.sh     # オーディオ変換
├── CMakeLists.txt        # ルートCMake設定
├── sdkconfig             # ESP-IDF設定（ビルド後生成）
├── sdkconfig.defaults    # デフォルトSDK設定
├── sdkconfig.defaults.esp32   # ESP32固有設定
├── sdkconfig.defaults.esp32s3 # ESP32-S3固有設定
├── sdkconfig.defaults.esp32c3 # ESP32-C3固有設定
├── dependencies.lock     # 依存関係ロック
└── README.md             # プロジェクトREADME
```

## 主要ファイルの説明

### main/main.cc
アプリケーションのエントリーポイント。`app_main()`関数がESP-IDFから呼び出されます。

### main/application.cc/h
メインアプリケーションロジック。シングルトンパターンで実装され、イベントループ、状態管理、オーディオ/ネットワーク処理を統合。

### main/device_state_machine.cc/h
デバイスの状態遷移を管理（アイドル、リスニング、シンキング、スピーキングなど）。

### main/mcp_server.cc/h
MCPプロトコルサーバー実装。デバイス制御（LED、サーボ、GPIOなど）のインターフェースを提供。

### main/audio/
オーディオ関連の全コンポーネント：
- `audio_service.cc/h`: オーディオサービスの統合
- `codecs/`: 各種オーディオコーデック（ES8311、ES8388など）のドライバー
- `processors/`: AFE（Acoustic Front-End）処理
- `wake_words/`: ウェイクワード検出ロジック

### main/boards/
各開発板の設定。各ボードディレクトリには：
- `<board>_board.cc`: ボード初期化コード
- `config.h`: ハードウェアピン定義
- `config.json`: ビルド設定（ターゲットチップ、SDKオプションなど）
- `README.md`: ボード説明

### main/display/
ディスプレイドライバーとLVGL統合コード。

### scripts/
ビルドとリリース用のPythonスクリプト。
- `release.py`: 特定のボード用にビルドし、リリースパッケージを作成
- `gen_lang.py`: 多言語JSONファイルからC++ヘッダーを生成
- `build_default_assets.py`: デフォルトアセット（フォント、絵文字など）をビルド

### CMakeLists.txt
ESP-IDFのCMakeベースビルドシステムの設定。プロジェクトバージョン、コンパイルオプション、コンポーネント定義を含む。

### sdkconfig*
ESP-IDFプロジェクト設定ファイル。`menuconfig`で編集可能。チップごとのデフォルト設定ファイルも提供。

## ボード設定の追加方法

新しいボードを追加するには：
1. `main/boards/<new-board>/` ディレクトリを作成
2. `config.h` でハードウェアピンを定義
3. `config.json` でビルド設定を定義
4. `<new-board>_board.cc` でボード初期化を実装
5. `main/CMakeLists.txt` に条件分岐を追加
6. `main/Kconfig.projbuild` に設定オプションを追加

詳細は `docs/custom-board.md` を参照。

## ビルド成果物

ビルド後、以下のディレクトリが生成されます：
```
build/
├── bootloader/           # ブートローダー
├── partition_table/      # パーティションテーブル
├── xiaozhi.bin           # アプリケーションバイナリ
├── merged-binary.bin     # マージされたフラッシュイメージ
└── compile_commands.json # コンパイルコマンド（IDE用）
```

## アセット管理

アセット（音声、フォント、絵文字）は2つの方法で管理：
1. **埋め込み**: `EMBED_FILES` でバイナリに直接埋め込み
2. **SPIFFSパーティション**: 独立したassetsパーティションに格納（v2）

v2では、カスタムアセットをassetsパーティションにフラッシュ可能。

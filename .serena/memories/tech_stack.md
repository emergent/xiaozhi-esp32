# 技術スタック

## 開発環境
- **IDE**: Cursor または VSCode
- **プラグイン**: ESP-IDF plugin
- **SDK**: ESP-IDF version 5.4以上（CIでは5.5を使用）
- **OS**: Linux推奨（Windowsよりコンパイルが高速でドライバー問題が少ない）

## プログラミング言語
- **主言語**: C++
- **コーディング規約**: Google C++ Code Style
- **標準**: C++11以上（標準ライブラリのstd::string、std::mutex、std::deque、std::memory等を使用）

## ビルドシステム
- **ビルドツール**: CMake（ESP-IDF標準のビルドシステム）
- **プロジェクト構成**: コンポーネントベースアーキテクチャ

## 対応チップ
- ESP32-C3
- ESP32-C5
- ESP32-C6
- ESP32-S3
- ESP32-P4

## 主要ライブラリ/フレームワーク
- **RTOS**: FreeRTOS（ESP-IDFに組み込み）
- **音声認識**: ESP-SR（Espressif Speech Recognition）
- **音声コーデック**: OPUS
- **オーディオコーデックドライバー**: ES8311, ES8374, ES8388, ES8389等
- **ディスプレイライブラリ**: LVGL（Light and Versatile Graphics Library）
- **通信プロトコル**: 
  - WebSocket
  - MQTT + UDP
- **ネットワーク**:
  - Wi-Fi
  - 4G（ML307 Cat.1モジュール対応）

## 開発ツール
- **idf.py**: ESP-IDFのメインコマンドラインツール
- **Python スクリプト**: 
  - `scripts/release.py`: ボード別ビルドとリリースパッケージ作成
  - `scripts/gen_lang.py`: 言語設定ファイル生成
  - `scripts/build_default_assets.py`: デフォルトアセットビルド
  - `scripts/mp3_to_ogg.sh`: オーディオ変換

## CI/CD
- GitHub Actions
- Docker コンテナ: `espressif/idf:release-v5.5`

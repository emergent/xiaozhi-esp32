# 推奨コマンド

## ESP-IDF開発コマンド

### 前提条件
ESP-IDFをインストールし、環境変数を設定する必要があります：
```bash
source $IDF_PATH/export.sh  # Linux/macOS
```

### ビルド
```bash
idf.py build
```
プロジェクト全体をビルドします。初回ビルド時は依存関係のダウンロードも行われます。

### フラッシュ（書き込み）
```bash
idf.py flash
```
ビルドしたファームウェアをESP32デバイスにフラッシュします。

### モニター
```bash
idf.py monitor
```
シリアルポート経由でデバイスのログ出力を監視します。Ctrl+]で終了。

### ビルド + フラッシュ + モニター（一括実行）
```bash
idf.py build flash monitor
```
ビルド、フラッシュ、モニターを一度に実行します。

### 設定メニュー
```bash
idf.py menuconfig
```
プロジェクトの設定を対話的に変更します（ボードタイプ、Wi-Fi設定、言語など）。

### クリーン
```bash
idf.py fullclean
```
ビルド成果物をすべて削除します。

### ボード選択とビルド
```bash
idf.py set-target esp32s3  # ターゲットチップを設定
idf.py menuconfig          # ボードタイプを選択
idf.py build
```

## リリースビルド

### 特定のボード用にビルド
```bash
python scripts/release.py <board_name>
```
例:
```bash
python scripts/release.py esp-box-3
python scripts/release.py m5stack-core-s3
```

### ボードリスト表示
```bash
python scripts/release.py --list-boards
```

### JSON形式でボードリスト表示（CI用）
```bash
python scripts/release.py --list-boards --json
```

### マージされたバイナリを作成
```bash
idf.py merge-bin
```
すべてのパーティション（ブートローダー、パーティションテーブル、アプリなど）を1つのバイナリにマージします。
出力: `build/merged-binary.bin`

## アセット関連コマンド

### MP3をOGGに変換
```bash
bash scripts/mp3_to_ogg.sh <input.mp3> <output.ogg>
```

### デフォルトアセットビルド
```bash
python scripts/build_default_assets.py --sdkconfig sdkconfig --output build/assets.bin
```

### 言語設定ファイル生成
```bash
python scripts/gen_lang.py --language ja-JP --output main/assets/lang_config.h
```

## Git操作

### ブランチ切り替え
```bash
git checkout v1    # v1ブランチに切り替え
git checkout main  # mainブランチ（v2）に切り替え
```

### リモート同期
```bash
git pull origin main
```

## macOS固有のコマンド

macOS（Darwin）では、標準のUnixコマンドが使用できます：
```bash
ls -la          # ディレクトリ一覧
cd <directory>  # ディレクトリ移動
grep <pattern>  # テキスト検索
find . -name    # ファイル検索
```

## デバッグ/開発支援

### シリアルポート確認（macOS）
```bash
ls /dev/cu.*    # 接続されているシリアルデバイスを確認
```

### ESP32デバイスのリセット
```bash
idf.py erase-flash  # フラッシュメモリを完全に消去
```

## CI/CDで使用されるコマンド

GitHub Actionsでは以下のコマンドが使用されています：
```bash
source $IDF_PATH/export.sh
python scripts/release.py <board_name> --name <variant_name>
```

## トラブルシューティング

### 依存関係の問題
```bash
idf.py reconfigure  # CMakeキャッシュを再構築
```

### ESP-IDFの再インストール/更新
ESP-IDFの公式ドキュメントを参照：
https://docs.espressif.com/projects/esp-idf/en/latest/get-started/

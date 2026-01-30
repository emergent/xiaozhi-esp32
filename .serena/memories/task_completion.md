# タスク完了時のワークフロー

## コード変更後の手順

### 1. ビルド確認
コードを変更した後は、必ずビルドが成功することを確認してください：
```bash
idf.py build
```

### 2. 対象ボードでのテスト
変更が特定のボード固有の場合、そのボードでテストしてください：
```bash
idf.py menuconfig  # ボードタイプを選択
idf.py build flash monitor
```

### 3. コードスタイルの確認
- Google C++ Code Styleに準拠していることを確認
- 命名規則（PascalCase、snake_case、UPPER_SNAKE_CASE）の遵守
- 適切なコメントの追加（英語で記述）
- インクルード順序の確認

### 4. コンパイル警告の解消
ビルド時に警告が出た場合は、可能な限り解消してください。

### 5. 機能テスト
- デバイスで実際に動作確認を行う
- 音声入出力、ディスプレイ、ネットワーク接続など、影響範囲をテスト

## コミット前

### コミットメッセージ
- 英語で記述
- 変更内容を簡潔に説明
- 例：
  ```
  Fix audio buffer overflow in ESP32-S3
  
  Add bounds checking to prevent buffer overflow when processing
  audio frames larger than expected.
  ```

### コミット対象の確認
不要なファイルをコミットしないように注意：
- `build/` ディレクトリ（ビルド成果物）
- `sdkconfig`（個人設定、デフォルトファイルのみコミット）
- IDE固有の設定ファイル

## プルリクエスト

### PR作成前
1. 最新のmainブランチとマージ
   ```bash
   git checkout main
   git pull origin main
   git checkout <your-branch>
   git merge main
   ```

2. コンフリクトを解決

3. ビルドが成功することを再確認

### PR説明
- 変更の目的と内容を明確に説明
- 関連するIssue番号を記載（存在する場合）
- テスト方法と結果を記載

### CI/CDの確認
GitHub ActionsでビルドCIが自動実行されます：
- 変更されたボードのみビルド（PR時）
- 全ボードのビルド（main へのpush時）
- CIが成功していることを確認

## リリースビルド

### 特定のボード用にリリースビルドを作成
```bash
python scripts/release.py <board_name>
```

これにより以下が実行されます：
1. ボード設定の適用
2. ビルド
3. マージされたバイナリの生成
4. ZIPパッケージの作成（`releases/`ディレクトリ）

### 複数ボードのビルド
```bash
python scripts/release.py --list-boards  # ボードリスト確認
# 各ボードごとにビルド
```

## デバッグ

### ログレベルの設定
`idf.py menuconfig` から：
- Component config -> Log output -> Default log verbosity

### シリアルモニターでのデバッグ
```bash
idf.py monitor
```
- Ctrl+]で終了
- ログ出力を確認

### コアダンプ解析
パニック時のコアダンプを解析：
```bash
idf.py coredump-info
```

## ドキュメント更新

コードに大きな変更を加えた場合：
1. 関連するREADMEを更新
2. `docs/`内のドキュメントを更新
3. コメント/ドキュメンテーションコメントを追加

## テスト環境

### 推奨テスト環境
- Linux（Ubuntu推奨）またはmacOS
- 実機ESP32デバイス
- ESP-IDF 5.4以上

### 実機テストが必要なケース
- 音声入出力の変更
- ディスプレイ関連の変更
- ネットワーク通信の変更
- 電源管理の変更
- ボード固有機能の変更

## リント/フォーマット

現在、自動リント/フォーマットツールは設定されていませんが、以下を手動で確認：
- Google C++ Code Styleの遵守
- ESP-IDFのコーディング規約に準拠
- 適切なエラーハンドリング（`ESP_ERROR_CHECK`の使用）

## 継続的改善

- CIで失敗した場合は、ローカルで再現して修正
- 警告を放置せず、可能な限り解消
- パフォーマンスやメモリ使用量に注意
- セキュリティ上の問題がないか確認

# コーディングスタイルと規約

## 全般的な規約
- **スタイルガイド**: Google C++ Code Style
- **コメント**: 英語で記述
- **エンコーディング**: UTF-8

## 命名規則

### クラス名
- **形式**: PascalCase
- **例**: `Application`, `DeviceState`, `AudioService`, `DeviceStateMachine`

### メソッド名
- **形式**: PascalCase
- **例**: `GetInstance()`, `Initialize()`, `Run()`, `SetDeviceState()`

### 関数名（Cスタイル）
- **形式**: snake_case
- **例**: `app_main()`

### 変数名
- **形式**: snake_case（ローカル変数、メンバー変数）
- **メンバー変数サフィックス**: アンダースコア `_`（例: `state_machine_`, `audio_service_`）

### 定数/マクロ
- **形式**: UPPER_SNAKE_CASE
- **例**: `MAIN_EVENT_SCHEDULE`, `AUDIO_INPUT_SAMPLE_RATE`, `DISPLAY_WIDTH`

### 列挙型
- **形式**: PascalCase（型名）、kPascalCase（値）
- **例**: 
```cpp
enum AecMode {
    kAecOff,
    kAecOnDeviceSide,
    kAecOnServerSide,
};
```

### ファイル名
- **ヘッダーファイル**: `.h` 拡張子
- **実装ファイル**: `.cc` 拡張子（C++）、`.c` 拡張子（C）
- **形式**: snake_case
- **例**: `application.h`, `application.cc`, `device_state_machine.h`

## ヘッダーファイル

### ヘッダーガード
```cpp
#ifndef _FILENAME_H_
#define _FILENAME_H_

// ヘッダーの内容

#endif  // _FILENAME_H_
```

### インクルード順序
1. ESP-IDF/FreeRTOSシステムヘッダー（`<esp_*.h>`, `<freertos/*.h>`, `<driver/*.h>`）
2. C++標準ライブラリ（`<string>`, `<mutex>`, `<memory>`, etc.）
3. プロジェクト内のヘッダー（`"filename.h"`）

### 例
```cpp
#include <freertos/FreeRTOS.h>
#include <freertos/event_groups.h>
#include <esp_timer.h>

#include <string>
#include <mutex>
#include <memory>

#include "protocol.h"
#include "audio_service.h"
```

## クラス設計

### シングルトンパターン
```cpp
class Application {
public:
    static Application& GetInstance() {
        static Application instance;
        return instance;
    }
    // Delete copy constructor and assignment operator
    Application(const Application&) = delete;
    Application& operator=(const Application&) = delete;
    
private:
    Application() = default;
};
```

## コメント

### ドキュメンテーションコメント
```cpp
/**
 * Initialize the application
 * This sets up display, audio, network callbacks, etc.
 * Network connection starts asynchronously.
 */
void Initialize();
```

### インラインコメント
```cpp
// Initialize NVS flash for WiFi configuration
esp_err_t ret = nvs_flash_init();
```

## フォーマット
- **インデント**: スペース4つ（または環境に応じた設定）
- **行の長さ**: 制限なし（可読性を優先）
- **ブレース**: K&Rスタイル（開きブレースは同じ行）

## 注意事項
- Google C++スタイルガイドに準拠してコードを提出すること
- ESP-IDFのAPIを使用する際は、エラーチェック（`ESP_ERROR_CHECK`）を適切に行うこと
- FreeRTOSのタスクやリソース管理に注意すること

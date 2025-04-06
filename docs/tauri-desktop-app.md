# Open WebUI デスクトップアプリケーション（Tauri）

このドキュメントでは、[Tauri](https://tauri.app/)を使用してOpen WebUIをデスクトップアプリケーションとしてビルドする方法を説明します。

## 前提条件

### すべてのプラットフォーム共通

- [Node.js](https://nodejs.org/) (v18.13.0以上)
- [npm](https://www.npmjs.com/) (v6.0.0以上)
- [Python](https://www.python.org/) (v3.8以上)
- [Rust](https://www.rust-lang.org/tools/install)

### Windowsの場合

- [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### macOSの場合

- Xcode Command Line Tools (`xcode-select --install`)

### Linuxの場合

- 必要なパッケージ（Ubuntuの例）:
  ```bash
  sudo apt update
  sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
  ```

## セットアップ手順

1. リポジトリをクローンし、依存関係をインストールします：

```bash
git clone https://github.com/open-webui/open-webui.git
cd open-webui
npm install
```

2. Pythonの依存関係をインストールします：

```bash
cd backend
pip install -r requirements.txt
cd ..
```

3. Tauriの依存関係をインストールします：

```bash
npm install -D @tauri-apps/cli
```

## アプリケーションのビルド

### 開発モード

開発モードでアプリケーションを実行するには：

```bash
npm run tauri:dev
```

これにより、開発サーバーが起動し、Tauriアプリケーションが開発モードで実行されます。

### 本番ビルド

本番用のデスクトップアプリケーションをビルドするには：

```bash
npm run tauri:build
```

ビルドが完了すると、以下の場所にアプリケーションが生成されます：

- Windows: `src-tauri/target/release/bundle/msi/`（.msiインストーラー）
- macOS: `src-tauri/target/release/bundle/dmg/`（.dmgインストーラー）
- Linux: `src-tauri/target/release/bundle/deb/`（.debパッケージ）または `src-tauri/target/release/bundle/appimage/`（.AppImageファイル）

## 注意事項

### バックエンドサーバー

デスクトップアプリケーションは、内部でPythonバックエンドサーバーを起動します。アプリケーションの起動時にバックエンドサーバーが自動的に起動し、アプリケーションの終了時に停止します。

### データの保存場所

デスクトップアプリケーションでは、データは以下の場所に保存されます：

- Windows: `%APPDATA%\open-webui\data`
- macOS: `~/Library/Application Support/open-webui/data`
- Linux: `~/.local/share/open-webui/data`

## トラブルシューティング

### バックエンドサーバーが起動しない場合

バックエンドサーバーが起動しない場合は、以下を確認してください：

1. Pythonがインストールされていることを確認します
2. 必要な依存関係がインストールされていることを確認します
3. ファイアウォールがアプリケーションをブロックしていないことを確認します

### ビルドエラーが発生する場合

ビルドエラーが発生する場合は、以下を確認してください：

1. Rustがインストールされていることを確認します
2. プラットフォーム固有の依存関係がインストールされていることを確認します
3. Node.jsとnpmのバージョンが要件を満たしていることを確認します

詳細なエラーログは、`src-tauri/target/debug/`または`src-tauri/target/release/`ディレクトリにあります。
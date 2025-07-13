# journal_observer

`journal_observer` は、`systemd-journald` のログをリアルタイムで監視し、**エラーメッセージなどの特定キーワードを含むログを Discord に通知する Rust 製サービス**です。

## ✨ 特徴

- `systemd` サービス単位でログを監視
- 任意のキーワードでフィルタリング
- Discord Webhook で即時通知
- `targets.yml` を非同期監視 → **設定ファイルの変更を即時反映（再起動不要）**
- `Ubuntu` や `Rocky Linux` に対応

---

## 🚀 使用方法

### 1. `targets.yml` を作成

```yaml
services:
  - name: sshd.service
    type: system
    keywords: ["Failed", "Disconnected", "error"]

  - name: test_echo.service
    type: system
    keywords: ["Error", "Alert"]
```

- `name`: 監視したい systemd サービス名
- `type`: `system` または `user`（user サービス監視に対応）
- `keywords`: 通知したいキーワード（複数指定可能）

> `targets.yml` はアプリ実行中でも自動で監視・再読み込みされるため、**再起動は不要です。**

---

### 2. 環境変数を設定

```env
DISCORD_WEBHOOK=https://discord.com/api/webhooks/XXXXXXXXXXXX
RUST_LOG=debug
TARGET_YML=/full/path/to/targets.yml
```

`.env` ファイルで定義するか、実行前に環境変数としてエクスポートしてください。

---

### 3. 実行方法

```bash
cargo run
```

※ `release` ビルドでの実行推奨：

```bash
cargo build --release
./target/release/journal_observer
```

---

## 🔧 動作環境

- Rust（1.70 以上推奨）
- Linux (Ubuntu, Rocky Linux など `systemd` 対応ディストリビューション)

---

## 🛡️ ライセンス

MIT License

---

# quies

**Mac を「静かで、既知の状態」に戻すための CLI ツール**

quies は macOS 専用の CLI ツールです。  
音声デバイスを単に切り替えるのではなく、  
**現在のオーディオ状態を「プロファイル」として保存し、確実に復元する**ことを目的としています。

---

## コンセプト

- 会議が終わったあと
- 外部マイク・ヘッドセットを抜き差ししたあと
- Bluetooth / USB デバイスの接続順が変わったあと
- macOS の音声設定が意図せず変わってしまったとき

そうした場面で、  
**Mac を「静かで、予測可能な状態」にスムーズに戻す**ためのツールです。

`quies` では、音声設定の状態を **プロファイル** として扱います。

プロファイルは「作業モード」を表します。

- `meeting`（会議用マイク・スピーカー）
- `desk`（デスク常設環境）
- `focus`（内蔵スピーカー・内蔵マイク）
- など

---

## 特徴

- macOS 専用
- Rust 製の軽量 CLI
- CoreAudio を直接利用（外部ツール不要）
- プロファイル中心の設計
- 常駐デーモンなし
- GUI なし（CLI のみ）
- `dry-run` による安全な差分確認

---

## インストール

> ⚠️ 現在は開発中です

現時点ではリポジトリを clone して利用します。

```bash
git clone https://github.com/yourname/quies.git
cd quies
cargo build --release
```

将来的には以下を予定しています：

- `cargo install`
- Homebrew formula

---

## 使い方

### プロファイル一覧
```bash
quies profile list
```

### プロファイル内容を表示
```bash
quies profile show <name>
```

### 現在のオーディオ状態を保存
```bash
quies profile save <name>
```

既存プロファイルを上書きする場合：
```bash
quies profile save <name> --force
```

### プロファイルを適用（差分確認のみ）
```bash
quies profile apply <name> --dry-run
```

### プロファイルを適用（実際に切り替え）
```bash
quies profile apply <name>
```

### プロファイルを削除
```bash
quies profile delete <name>
```

---

## dry-run について

`--dry-run` を付けると、**実際には変更を行わず**、  
現在の状態とプロファイルとの差分だけを表示します。

---

## プロファイルについて

プロファイルは JSON 形式で保存されます。

### 保存先
```
~/Library/Application Support/quies/profiles/
```

### 含まれる情報
- プロファイル名
- version（将来のマイグレーション用）
- デフォルト出力デバイス UID
- デフォルト入力デバイス UID

---

## 非ゴール

v1 では以下を目的としていません：

- リアルタイム監視
- 常駐プロセス（デーモン）
- メニューバーアプリ
- GUI 操作
- 自動切り替え

---

## ステータス

このプロジェクトは **アクティブに開発中**です。

## 連携
- [Hammerspoon を使った quies の自動プロファイル適用](docs/hammerspoon.md)

---

## ライセンス

MIT OR Apache-2.0

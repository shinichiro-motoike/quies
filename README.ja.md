# quies

**Mac を「静かで、既知の状態」に戻すための CLI ツール**

quies は macOS 専用の CLI ツールです。  
音声デバイスを単に切り替えるのではなく、  
**現在の音声デバイス状態を「プロファイル」として保存・復元**することに焦点を当てています。

---

## コンセプト

quies は「音を切り替えるツール」ではありません。

- 会議が終わったあと
- 外部音声デバイスを抜き差ししたあと
- 何らかの理由で音声設定が壊れてしまったとき

そんな場面で、**Mac を「静かで、予測可能な状態」に戻す**ためのツールです。

プロファイルは「作業モード」を表します。

- `meeting`
- `desk`
- `focus`
- など

---

## 特徴

- macOS 専用
- Rust 製の軽量 CLI
- CoreAudio を直接利用
- プロファイル中心の設計
- 常駐デーモンなし
- GUI なし（CLI のみ）

---

## インストール

> ⚠️ 現在は開発中です

将来的には以下の方法を想定しています：

- Homebrew
- `cargo install`

---

## 使い方（予定）

```bash
quies profile list
quies profile show <name>
quies profile save <name>
quies profile apply <name> [--dry-run]
quies profile delete <name>
```

---

## プロファイルについて

プロファイルは JSON 形式で保存されます。

- 保存先（予定）  
  `~/Library/Application Support/quies/profiles/`

- プロファイルには以下が含まれます：
  - 音声デバイス情報
  - 将来互換のための version フィールド

---

## 非ゴール

v1 では以下を目的としていません：

- リアルタイム監視
- 常駐プロセス（デーモン）
- メニューバーアプリ
- GUI 操作

---

## ステータス

このプロジェクトは **開発初期段階**です。  
API や内部仕様は今後変更される可能性があります。

---

## ライセンス

MIT OR Apache-2.0

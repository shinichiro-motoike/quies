# Repository Guidelines

## プロジェクト構成とモジュール

- `crates/quies-cli/`: CLI のエントリポイントとコマンド配線（`src/main.rs`）。
- `crates/quies-core/`: プロファイル処理と CoreAudio 連携のコアライブラリ（`src/lib.rs`, `src/coreaudio.rs`, `src/profile.rs`）。
- `docs/`: 追加ドキュメントや連携ガイド（例: Hammerspoon）。
- ルートの `Cargo.toml`: ワークスペース定義と共有依存関係。

## ビルド・テスト・開発コマンド

- `cargo build`: ワークスペース全体をデバッグビルド。
- `cargo build --release`: 最適化済みバイナリをビルド。
- `cargo run -p quies-cli -- <args>`: CLI をローカル実行（例: `cargo run -p quies-cli -- profile list`）。
- `cargo test`: テスト実行（現状は最小/未整備）。

## コーディングスタイルと命名

- Rust 2021 edition（ワークスペース既定）。
- インデントは 4 スペース。標準的な Rust 書式に準拠。
- 重要な変更時は `rustfmt` で整形し、`clippy` で lint。
- 命名: モジュールは `snake_case`、型は `PascalCase`、関数/変数は `snake_case`。

## テスト指針

- 単体テストは対象モジュールに併設（`.rs` 内の `mod tests`）。
- テスト関数は `test_` 接頭辞（例: `test_save_profile`）。
- 変更提出前に `cargo test` を実行。

## コミット & PR ガイドライン

- コミットメッセージは `feat:` / `fix:` などの `type:` 接頭辞が一般的。
- 変更は小さく焦点化し、必要なら本文で背景を補足。
- PR には概要、関連 Issue（あれば）、検証手順を記載。
- 仕様変更時は CLI 実行例や注意点を添える（例: `quies profile apply <name>`）。

## セキュリティと設定の注意

- プロファイルは `~/Library/Application Support/quies/profiles/` に JSON で保存。
- ローカルのプロファイルや機器固有データはコミットしない。
- macOS/CoreAudio 前提のため、プラットフォーム依存変更はガードと記述を追加。

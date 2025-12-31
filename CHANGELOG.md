# Changelog

## [0.14.0] - 2025-12-31

### 🎉 New Features
- **キャッシュデフォルト有効化**: キャッシュがデフォルトで有効になり、CLIベストプラクティスに準拠
- **SARIF出力統一**: Claude Code agentの出力をSARIF形式に統一
- **ファイル直接出力モード**: claude-code agentに直接ファイル出力モードを追加

### 🐛 Bug Fixes
- SARIFファイル読み込み前の存在確認を追加
- Ctrl+Cシグナルハンドリングの改善
- nix-profile git binaryのcredential helper対応
- ログメッセージから不要な括弧を削除
- 未使用変数の警告を抑制

### 🔧 Improvements
- GitHub操作をgithub.rsモジュールに統合
- レガシーなanalyze_with_claude_code関数を削除
- LLMキャッシュファイルをgit管理から除外

---

## [0.11.0] - 2025-12-27

### 🎉 New Features
- **MVRA (Multi-Repository Variant Analysis)**: 複数リポジトリに対するvariant analysis機能を実装
- **MVRAモード自動検出**: ターゲット引数からMVRAモードを自動検出
- **Claude Code パターン生成**: --generate-patternsでClaude Codeをサポート

### 🐛 Bug Fixes
- Claude Code JSON レスポンスのnull値処理を修正
- ネストされたJSON構造全体のnull値処理を修正
- CI disk cleanup処理を改善

### 🔧 Improvements
- MVRA CLIオプションの簡素化
- Claude Codeパターンジェネレーターをメインクレートに移動
- GitHub tokenのgit credential helper使用に変更
- UI出力の可視性向上（冗長な出力を削減）

---

## [0.10.0] - 2025-12-27

### 🎉 New Features
- MVRA基盤の追加

---

## [0.9.2] - 2025-12-27

### 🎉 New Features
- **デフォルトモデル変更**: o4-mini から gpt-5.1-codex へ更新
- **デフォルト出力ディレクトリ**: ./reports に設定

### 📚 Documentation
- README を大幅更新し、可読性を向上
- README 見出しレベルを調整
- PAR 用語の説明を改善
- ベンチマークコマンドのドキュメント整備（--root フラグ削除）
- CLI ドキュメントの更新と Docker 参照削除

### 🛠️ Infrastructure
- **CI/CD 簡素化**: Docker 関連ファイルを削除
- claude.yml ワークフロー削除
- ghcr deploy ワークフロー削除

### 🎨 UI/UX
- ウェブサイトテーマを紫からグレースケールに変更

### ✅ Testing
- 全ユニットテスト (29 tests) 合格
- ビルド確認完了

---

## [0.9.1] - Previous Release

[Previous release notes...]

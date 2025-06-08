# LLM解析用context構築

本文書では、Parsentryによる正確な脆弱性検出を実現するためのLLM context構築手法を説明します。

## 概要

正確な脆弱性検出には、個別codeセグメントの解析に加え、実行環境を含む包括的contextの理解が必要です。Parsentryは多層contextアーキテクチャによりLLMの解析精度を向上させます。

## context層

### 1. プロジェクトレベルcontext

#### README解析
- プロジェクトREADMEファイルを自動抽出・要約
- 以下の理解を提供：
  - プロジェクトの目的と機能
  - 技術stackとdependency
  - 開発者によって言及されたsecurity考慮事項
  - API endpointとdata flow

#### repository構造
- プロジェクトdirectory構造をmapping
- 以下を識別：
  - entry point（mainファイル、API route）
  - 設定ファイル
  - test directory
  - third-party dependency

#### dependency解析
- package manifest（package.json、Cargo.toml、requirements.txt等）を解析
- 既知の脆弱なdependencyを識別
- dependency使用patternをmapping

### 2. ファイルレベルコンテキスト

#### ソースコード解析
- 完全なファイル内容をLLMに提供
- 以下を保持：
  - 元のフォーマットとインデント
  - コメントとドキュメント
  - インポート文
  - 関数/クラス構造

#### セマンティック情報
Tree-sitter解析を使用：
- **関数定義**: 名前、パラメータ、戻り値型
- **変数宣言**: スコープと使用パターン
- **制御フロー**: 条件分岐、ループ、エラーハンドリング
- **データフロー**: ユーザー入力がコード内で伝播する方法

#### ファイルメタデータ
- プロジェクト内の相対パス
- ファイルタイプと言語
- 最終変更時刻
- ファイルサイズと複雑度メトリクス

### 3. コード関係コンテキスト

#### インポート解析
- インポートされたモジュールと関数を追跡
- クロスファイル依存関係をマップ化
- 外部ライブラリ使用を識別

#### 関数呼び出しグラフ
- ファイル間の関数呼び出しを追跡
- 以下を識別：
  - ユーザー入力のエントリーポイント
  - データ変換ポイント
  - セキュリティ重要関数

#### データフロー追跡
- 関数境界を越えた変数を追跡
- 以下を追跡：
  - ユーザー入力ソース
  - データ変換
  - 出力先

## コンテキスト構築プロセス

### 1. 初期スキャン
```
リポジトリ → ファイル発見 → 言語検出 → パターンマッチング
```

### 2. 解析段階
```
ソースファイル → Tree-sitter AST → セマンティック抽出 → 関係マッピング
```

### 3. コンテキスト組み立て
```
プロジェクト情報 + ファイル内容 + セマンティックデータ → 構造化コンテキスト
```

## コンテキスト最適化

### 関連性フィルタリング
- セキュリティ関連コードセクションに焦点
- 以下を優先：
  - 入力処理関数
  - データベースクエリ
  - ファイル操作
  - ネットワークリクエスト
  - 認証/認可コード

### コンテキストウィンドウ管理
- 大きなファイルを適切に切り詰め
- 重要セクションを保持：
  - 関数シグネチャ
  - セキュリティ重要操作
  - エラーハンドリング
  - 入力検証

### クロスリファレンス強化
- 他ファイルからの関連コードを含める
- 呼び出される関数の定義を追加
- 関連する設定値を含める

## 実装詳細

### Tree-sitterクエリ

関数定義と参照抽出のクエリ例：
```scheme
; 定義クエリ（definitions.scm）
(function_definition
  name: (identifier) @name
  body: (block)) @definition

; 参照クエリ（references.scm）
(identifier) @reference
```

現在サポートされる言語：C、C++、Python、JavaScript、TypeScript、Java、Go、Rust、Ruby

### コンテキストテンプレート構造

```rust
// Definition構造体
pub struct Definition {
    pub name: String,
    pub start_byte: usize,
    pub end_byte: usize, 
    pub source: String,
}

// Context構造体
pub struct Context {
    pub definitions: Vec<Definition>,
}
```

プロンプトで使用される実際のコンテキスト形式：
```
Context Definitions:

Function/Definition: function_name
Code:
function_body_source_code
```

## ベストプラクティス

### 1. 包括的カバレッジ
- すべての関連コンテキスト層を含める
- プロジェクト固有の事項についてLLMの知識を仮定しない
- 明示的にセキュリティ関連情報を提供

### 2. ノイズ削減
- 無関係なコード（テスト、ドキュメント）をフィルタリング
- 実行可能コードパスに焦点
- ユーザー向け機能を優先

### 3. 関係の明確性
- コード関係を明示的に記述
- データフローパスを強調
- 信頼境界をマーク

## 将来の機能強化

1. **動的解析統合**
   - 実行時動作パターン
   - 実際のデータフロートレース
   - パフォーマンス特性

2. **履歴コンテキスト**
   - Git履歴解析
   - 以前の脆弱性修正
   - コード進化パターン

3. **外部コンテキスト**
   - CVEデータベース統合
   - セキュリティアドバイザリ相関
   - フレームワーク固有脆弱性

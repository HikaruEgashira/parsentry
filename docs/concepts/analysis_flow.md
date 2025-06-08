# LLM解析flow

本文書では、Parsentryがlarge language modelを活用してsource codeのsecurity解析を実行する方法について説明します。

## 概要

解析processは、静的code解析とLLMを使用した脆弱性検出を組み合わせています。systemは最初にsecurityパターンmatchingを使用して潜在的に脆弱なfileを特定し、その後LLMを使用して深度解析を実行し、脆弱性を確認・特性化します。

## 解析pipeline

### 1. file発見とfiltering

- repositoryがsource fileをscan
- fileは以下の基準でfiltering：
  - 言語support（Rust、Python、JavaScript/TypeScript、Ruby、Go、Java、C/C++、Terraform）
  - `src/patterns/`directoryで言語別に定義されたsecurityリスクpattern
  - fileサイズと複雑度の閾値

### 2. patternベースリスク評価

- 各fileは言語固有のsecurityパターン（PAR分類）に対して評価
- リスクscoreは以下に基づいて計算：
  - Principal（dataソース）パターン：入力源、requestハンドラー、環境変数等
  - Action（操作）パターン：data検証、sanitization、hash化等
  - Resource（リソース）パターン：file操作、database、command実行等
- MITRE ATT&CK frameworkに基づく攻撃vectorの関連付け

### 3. code context構築

- Tree-sitterがsource codeを解析して以下を抽出：
  - 関数/method定義
  - 変数参照とdata flow
  - import文とdependency
  - commentとdocument
- semantic情報は潜在的脆弱性の周辺context構築に使用

#### 改進されたcontext追跡

PAR（Principal-Action-Resource）分類systemにより、context収集が最適化されました：

- **Principalパターン**: `find_references()` を使用してdataの流れを前方追跡
- **Actionパターン**: `find_bidirectional()` を使用してdata処理の前後両方向を追跡
- **Resourceパターン**: `find_definition()` を使用して定義を後方追跡  
- **攻撃vector**: MITRE ATT&CK tacticsに基づく脅威の分類

これにより、より正確なdata flow解析と脆弱性のcontext理解が可能になります。

### 4. LLM解析

#### 初期解析

1. **prompt構築**：
   - security解析guidelineを含むsystem prompt
   - 対象fileの完全なsource code
   - project context（READMEサマリーがある場合）
   - JSON形式出力の具体的指示

2. **LLM request**：
   - API clientが選択されたmodel（OpenAI、Anthropic等）にrequest送信
   - modelが脆弱性patternのcode解析を実行
   - responseには脆弱性評価が含まれる

3. **response解析**：
   - JSON responseがschemaに対して検証
   - 抽出されるfieldには以下が含まれる：
     - 特定された脆弱性type
     - 詳細解析
     - 概念実証code
     - 信頼度score
     - 修復提案

#### 深度脆弱性解析（オプション）

特定された脆弱性に対して、システムは標的解析を実行可能：

1. **脆弱性固有プロンプト**：
   - 各脆弱性タイプの専用プロンプトを取得
   - 既知のバイパス技術とエッジケースを含める
   - 悪用可能性評価に焦点を当てる

2. **反復的改善**：
   - 脆弱性固有コンテキストで再解析
   - より深い解析に基づいて信頼度スコアを更新
   - より正確な概念実証コードを生成

### 5. 結果集約

- 解析されたすべてのファイルからの発見を結合
- 信頼度スコアと重要度でソート
- マークダウンまたはJSONレポートとして出力をフォーマット

## 主要コンポーネント

### コアモジュール

- **`src/analyzer.rs`**: メイン解析オーケストレーション
- **`src/prompts/`**: LLMプロンプトテンプレートとガイドライン
- **`src/response.rs`**: レスポンススキーマと検証
- **`src/parser.rs`**: コード解析のためのTree-sitter統合
- **`src/security_patterns.rs`**: パターンマッチングエンジン

### 外部依存関係

- **`genai`**: LLM APIクライアント抽象化
- **`tree-sitter`**: コード解析とAST生成
- **`serde_json`**: JSONシリアライゼーション/デシリアライゼーション

## 設定

### モデル選択

サポートされるモデル：
- OpenAI: gpt-4、gpt-4-turbo、gpt-3.5-turbo
- Anthropic: claude-3-opus、claude-3-sonnet、claude-3-haiku
- Google: gemini-pro
- Groq: llama等の高速推論モデル
- ローカルモデル（Ollama等との互換）

### 解析パラメータ

- **最大ファイル数**: 解析するファイル数の制限
- **タイムアウト**: APIリクエストタイムアウト設定
- **信頼度閾値**: レポート出力の最小スコア
- **パターン感度**: パターンマッチング厳密度の調整

## パフォーマンス考慮事項

1. **並列処理**: 複数ファイルの同時解析
2. **キャッシュ**: 再解析を避けるための結果キャッシュ
3. **インクリメンタル解析**: 変更されたファイルのみ解析
4. **モデル選択**: 精度とコスト/速度のバランス

## セキュリティ注意事項

- すべての解析はローカルまたはセキュアAPIを通じて実行
- 解析以外でコードの保存や送信は行わない
- APIキーは適切にセキュア化する必要がある
- 結果はセキュリティ専門家によってレビューされるべき

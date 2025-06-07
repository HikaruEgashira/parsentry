# LLMプロンプトドキュメント

このドキュメントでは、Parsentryが正確なセキュリティ解析を実行するためにLLMを誘導するために使用するプロンプトエンジニアリングアプローチについて説明します。

## プロンプトアーキテクチャ

Parsentryは以下を目的とした多層プロンプトシステムを使用します：
1. セキュリティ解析コンテキストを確立
2. 体系的脆弱性発見を誘導
3. 一貫性のある実用的な出力を保証
4. 脆弱性固有の深堀りを可能にする

## コアプロンプト

### 1. システムプロンプト

**目的**: LLMの役割をセキュリティ研究者として確立し、出力要件を定義

**主要コンポーネント**:
- 役割定義: "あなたは専門のセキュリティ研究者です"
- 焦点領域:
  - 入力検証とサニタイゼーション
  - 認証と認可
  - データ処理と保存
  - ネットワーク操作
  - コマンド実行
  - ファイル操作
- 厳密なJSON出力スキーマ強制
- 悪用可能な脆弱性のみに重点

**Output Schema**:
```json
{
  "scratchpad": "Analysis thought process",
  "analysis": "Detailed vulnerability explanation", 
  "poc": "Proof of concept code",
  "confidence_score": 0-10,
  "vulnerability_types": ["LFI", "RCE", "SSRF", "AFO", "SQLI", "XSS", "IDOR"],
  "context_code": [
    {
      "name": "function/method name",
      "reason": "why this code is vulnerable",
      "code_line": "actual vulnerable code",
      "path": "file path"
    }
  ]
}
```

### 2. システムプロンプト（実装済み）

**実際のシステムプロンプト**:
```
You are a security vulnerability analyzer. You must reply with exactly one JSON object that matches this schema: { "scratchpad": string, "analysis": string, "poc": string, "confidence_score": integer, "vulnerability_types": array of strings, "context_code": array of objects with { "name": string, "reason": string, "code_line": string } }. Do not include any explanatory text outside the JSON object.
```

**日本語ガイドライン**:
- セキュリティ研究者として、コードの脆弱性を分析
- 入力値の検証とサニタイズに注目
- 認証・認可の確認
- データの取り扱いと漏洩
- コマンドインジェクションの可能性
- パストラバーサルの脆弱性

### 3. 初期解析プロンプト

**目的**: 提供されたコードの幅広いセキュリティスイープを実行

**解析領域**:
- **入力処理**: ユーザー入力、フォームデータ、APIパラメータ
- **認証**: セッション管理、トークン検証
- **ファイル操作**: パストラバーサル、ファイルインクルージョンリスク
- **データベースクエリ**: SQLインジェクション脆弱性
- **コマンド実行**: OSコマンドインジェクション
- **ネットワーク操作**: SSRF、リクエスト偽造

**コンテキスト統合**:
- プロジェクトサマリーを含む
- 対象ファイルの完全なソースコード
- ファイルパスとプロジェクト構造ヒント

### 4. 脆弱性固有プロンプト（実装済み）

各脆弱性タイプに対して専用のバイパス技術が定義済み：

#### ローカルファイルインクルージョン (LFI)
- Path traversal sequences(../../)
- URL encoding
- Null byte injection

#### リモートコード実行 (RCE) 
- Shell metacharacters for command injection
- Python execution vectors
- Deserialization attacks

#### SQLインジェクション (SQLI)
- UNION-based injection
- Boolean-based blind injection
- Time-based blind injection

#### クロスサイトスクリプティング (XSS)
- HTML entity encoding bypass
- JavaScript template injection
- DOM-based XSS vectors

#### サーバーサイドリクエストフォージェリ (SSRF)
- DNS rebinding
- IP address encoding tricks
- Redirect chain

#### 任意ファイル操作 (AFO)
- Directory traversal sequences
- Following symbolic links
- Race conditions

#### 安全でない直接オブジェクト参照 (IDOR)
- Parameter tampering
- Horizontal privilege escalation
- Predictable resource paths

### 5. 解析ガイドライン

**方法論指示**:
1. **データフロートレース**: ユーザー入力をエントリから処理まで追跡
2. **信頼境界解析**: 検証が発生する場所を識別
3. **影響評価**: 悪用可能性と重要度を決定
4. **バイパス考慮**: 攻撃者のように考える
5. **信頼度評価**: コードの明確性と悪用可能性に基づく

**品質基準**:
- 悪用可能な脆弱性のみレポート
- 具体的なコード参照を提供
- 動作する概念実証を含める
- 攻撃シナリオを明確に説明
- 実用的な修復策を提案

### 6. 評価プロンプト

**目的**: テストシナリオで解析品質を評価

**評価メトリクス**:
- 真陽性率
- 偽陽性率
- PoC有効性
- 解析完全性
- 修復品質

## プロンプトエンジニアリングベストプラクティス

### 1. 明確性と具体性
- 正確な技術用語を使用
- 明確な例を提供
- 正確な出力形式を定義
- 曖昧な指示を回避

### 2. コンテキスト管理
- 関連するコードコンテキストを含める
- プロジェクトレベルの理解を提供
- セキュリティに焦点を維持
- 詳細と簡潔性のバランス

### 3. 出力の一貫性
- 構造化形式を強制
- スキーマ検証を使用
- 特定のフィールドを要求
- 信頼度スコアリングを標準化

### 4. 反復的改善
- 既知の脆弱性でプロンプトをテスト
- 偽陽性率に基づいて調整
- 新しい攻撃パターンを組み込み
- 新興脅威に対して更新

## 高度な技術

### 思考連鎖プロンプティング
"scratchpad"フィールドは段階的推論を促進:
```
1. 入力ソースを識別
2. データフローを追跡
3. 検証ギャップを発見
4. 悪用を構築
5. 悪用可能性を検証
```

### フューショット例
プロンプトで脆弱性例を提供:
```python
# 脆弱:
query = f"SELECT * FROM users WHERE id = {user_input}"

# 悪用:
user_input = "1 OR 1=1--"
```

### ネガティブ例
報告すべきでないものを明確化:
```
- PoCなしの理論的脆弱性
- 特権アクセスが必要な問題
- セキュリティ影響のないベストプラクティス違反
```

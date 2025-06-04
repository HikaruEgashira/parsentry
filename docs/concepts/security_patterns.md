# セキュリティパターンドキュメント

このドキュメントでは、Vulnhuntrsが複数のプログラミング言語にわたってセキュリティに敏感なコードを識別するために使用するパターンベースの脆弱性検出システムについて説明します。

## 概要

セキュリティパターンは脆弱性検出の第一線を担い、高コストなLLM解析の前に潜在的に脆弱なコードを効率的にフィルタリングします。システムは、サポートされる各言語に最適化された正規表現を使用してセキュリティクリティカルな操作を識別します。

## パターンカテゴリ

### 1. 入力処理パターン

外部入力を処理するコードを識別：

#### Webフレームワーク
- **Python**: Flask (`@app.route`)、FastAPI (`@router.post`)、Django (`views.py` パターン)
- **JavaScript/TypeScript**: Express (`app.get/post`)、Next.js API ルート
- **Ruby**: Rails コントローラー、Sinatra ルート
- **Go**: HTTP ハンドラー (`http.HandleFunc`)
- **Java**: Spring (`@RequestMapping`)、Servlets
- **Rust**: Actix-web (`#[post]`)、Rocket ルート

#### パターン例
```yaml
python:
  - pattern: "@app\\.route\\("
    risk_score: 7
    category: "web_handler"
    description: "Flask ルートハンドラー"
    
  - pattern: "request\\.(args|form|json|data)"
    risk_score: 8
    category: "input_access"
    description: "直接的なリクエストデータアクセス"
```

### 2. データベース操作

潜在的なSQLインジェクションベクターを検出：

#### クエリ構築
- 生SQLクエリ: `SELECT`、`INSERT`、`UPDATE`、`DELETE`
- クエリ内の文字列補間
- 動的クエリビルダー
- ORM生クエリ

#### パターン例
```yaml
generic:
  - pattern: "SELECT.*?FROM.*?WHERE"
    risk_score: 6
    category: "sql_query"
    description: "SQL SELECT文"
    
  - pattern: "execute\\([\"'].*?%[sd]"
    risk_score: 9
    category: "sql_injection"
    description: "SQL内の文字列フォーマット"
```

### 3. ファイルシステム操作

パストラバーサルリスクを識別：

#### ファイルアクセス
- ファイル開く/読み取り/書き込み操作
- パス操作関数
- ディレクトリトラバーサルパターン
- ファイルアップロードハンドラー

#### パターン例
```yaml
python:
  - pattern: "open\\(.*?\\)"
    risk_score: 7
    category: "file_operation"
    description: "ファイル開く操作"
    
javascript:
  - pattern: "fs\\.(readFile|writeFile|readdir)"
    risk_score: 8
    category: "file_operation"
    description: "Node.js ファイルシステムアクセス"
```

### 4. コマンド実行

OSコマンドインジェクションリスクを検出：

#### シェル操作
- 直接的なシェル実行
- プロセス生成
- システムコマンド
- パイプライン操作

#### パターン例
```yaml
python:
  - pattern: "subprocess\\.(run|call|Popen)"
    risk_score: 9
    category: "command_execution"
    description: "サブプロセス実行"
    
ruby:
  - pattern: "system\\(|`.*?`|exec\\("
    risk_score: 9
    category: "command_execution"
    description: "システムコマンド実行"
```

### 5. ネットワーク操作

SSRFと外部通信を識別：

#### HTTPリクエスト
- HTTPクライアント使用
- URL構築
- 外部API呼び出し
- Webhookハンドラー

#### パターン例
```yaml
python:
  - pattern: "requests\\.(get|post|put|delete)"
    risk_score: 6
    category: "network_request"
    description: "requestsライブラリを使用したHTTPリクエスト"
    
javascript:
  - pattern: "fetch\\(|axios\\."
    risk_score: 6
    category: "network_request"
    description: "HTTPクライアント使用"
```

### 6. 認証と認可

セキュリティクリティカルな認証操作：

#### 認証パターン
- ログインハンドラー
- セッション管理
- トークン検証
- 権限チェック

#### パターン例
```yaml
generic:
  - pattern: "password|passwd|secret|token"
    risk_score: 7
    category: "sensitive_data"
    description: "潜在的な機密データ処理"
    
  - pattern: "jwt\\.(sign|verify|decode)"
    risk_score: 8
    category: "authentication"
    description: "JWTトークン操作"
```

### 7. 危険な関数

既知の安全でない操作：

#### 高リスク関数
- `eval()`、`exec()` - コード実行
- `dangerouslySetInnerHTML` - XSSリスク
- デシリアライゼーション関数
- リフレクション/イントロスペクション

#### パターン例
```yaml
javascript:
  - pattern: "eval\\(|Function\\(|setTimeout\\([^,]+,"
    risk_score: 10
    category: "code_execution"
    description: "動的コード実行"
    
python:
  - pattern: "pickle\\.loads|yaml\\.load\\("
    risk_score: 9
    category: "deserialization"
    description: "安全でないデシリアライゼーション"
```

## パターン設定

### リスクスコアリング

各パターンにはリスクスコア (1-10) があります：
- **1-3**: 低リスク、情報提供
- **4-6**: 中リスク、コンテキストが必要
- **7-9**: 高リスク、脆弱の可能性が高い
- **10**: クリティカル、ほぼ確実に脆弱

### パターン構造

```yaml
languages:
  <language>:
    file_patterns:
      - pattern: "regex_pattern"
        risk_score: <1-10>
        category: "<category_name>"
        description: "人間が読める説明"
        context_lines: 5  # オプション: キャプチャする行数
```

## 実装詳細

### パターンマッチングエンジン

1. **コンパイル**: パフォーマンスのために起動時にパターンをコンパイル
2. **マッチング**: マルチスレッド並列マッチング
3. **スコアリング**: ファイルごとの累積リスクスコア
4. **フィルタリング**: 閾値未満のファイルをスキップ

### パフォーマンス最適化

- **遅延評価**: 最初の高リスクマッチで停止
- **パターン順序**: 最も一般的なパターンを最初に
- **キャッシュ**: 変更されていないファイルの結果をキャッシュ
- **並列処理**: 複数ファイルを同時に解析

## ベストプラクティス

### 効果的なパターンの作成

1. **特異性**: 偽陽性とカバレッジのバランス
2. **コンテキスト**: 曖昧さを減らすために十分なコンテキストを含める
3. **パフォーマンス**: 破滅的バックトラッキングを避ける
4. **メンテナンス**: パターンの目的と例を文書化

### パターンテスト

```rust
#[test]
fn test_sql_injection_pattern() {
    let pattern = compile_pattern(r"execute\([\"'].*?%[sd]");
    assert!(pattern.is_match("cursor.execute('SELECT * FROM users WHERE id = %s' % user_id)"));
    assert!(!pattern.is_match("cursor.execute('SELECT * FROM users WHERE id = ?', [user_id])"));
}
```

## カスタマイゼーション

### カスタムパターンの追加

1. `security_patterns/patterns.yml`を編集
2. 言語固有セクションを追加
3. メタデータとともにパターンを定義
4. 既知の脆弱性に対してテスト

### パターンの無効化

```yaml
# 特定のパターンを無効化
disabled_patterns:
  - "eval\\("  # このコードベースで偽陽性が多すぎる
```

### プロジェクト固有パターン

プロジェクトルートに`.vulnhuntrs/patterns.yml`を作成：
```yaml
custom_patterns:
  python:
    - pattern: "custom_dangerous_function"
      risk_score: 8
      category: "custom"
      description: "プロジェクト固有の危険な関数"
```

## LLMとの統合

パターンは複数の目的を果たします：

1. **ファイルフィルタリング**: マッチしないファイルをスキップ
2. **コンテキスト抽出**: マッチした領域をキャプチャ
3. **焦点ガイダンス**: LLMの注意を向ける
4. **優先度スコアリング**: 高リスクファイルを最初に解析

## 将来の機能強化

1. **セマンティックパターン**: ASTベースのマッチング
2. **データフロー解析**: 汚染された変数の追跡
3. **機械学習**: プロジェクト固有パターンの学習
4. **インタラクティブチューニング**: ユーザーフィードバックの統合

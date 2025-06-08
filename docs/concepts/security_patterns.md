# security pattern

Parsentryが使用するpatternベースの脆弱性検函systemの概念について説明します。

## 概要

security patternは、LLM解析の前段階で潜在的な脆弱性を効率的にfilteringする正規表現ベースのruleです。複数のprogramming言語に対応し、data flow解析と組み合わせて使用されます。

## PAR（Principal-Action-Resource）分類

### Principal
認証主体や信頼できるdataソースを識別します。

**programming言語における例**:
- **Web framework**: HTTP requestハンドラー、form data処理
- **file system**: file読み取り、設定file解析
- **network**: API応答、外部service呼び出し
- **環境**: 環境変数、commandライン引数、database結果

**IaCにおける例**:
- **AWS IAM**: ユーザー、ロール、アカウント
- **アクセス権限**: ポリシーアタッチメント

### Action
data処理、検証、security制御を表す操作を識別します。

**programming言語における例**:
- **data検証**: schema検証、型check、正規表現検証
- **data sanitization**: HTMLエスケープ、path正規化
- **security制御**: hash化、暗号化、token検証

**IaCにおける例**:
- **AWS API操作**: s3:GetObject、ec2:DescribeInstances
- **権限変更**: IAMポリシー更新

### Resource
dataの最終的な出力先や危険な操作対象を識別します。

**programming言語における例**:
- **code実行**: `eval()`、`exec()`、動的code実行
- **command実行**: shell実行、process生成
- **database**: SQL query実行、NoSQL操作
- **file system**: file書き込み、path traversal
- **network**: 外部HTTP request、URL構築

**IaCにおける例**:
- **AWS サービス**: S3バケット、EC2インスタンス、Lambda関数
- **ネットワークリソース**: VPC、サブネット、セキュリティグループ

## データフロー解析との統合

パターンマッチングの結果に基づいて、適切なコンテキスト抽出を行います：

- **Principalマッチ**: `find_references()`を使用してデータの流れを前方追跡
- **Action/Resourceマッチ**: `find_definition()`を使用してデータの起源を後方追跡
- **攻撃ベクター**: MITRE ATT&CKフレームワークのタクティクスIDで脅威を分類

## リスクスコアリング

各パターンには1-10のリスクスコアが付与されます：

- **1-3**: 低リスク（情報提供）
- **4-6**: 中リスク（コンテキスト依存）
- **7-9**: 高リスク（脆弱性の可能性が高い）
- **10**: クリティカル（ほぼ確実に脆弱）

## 解析パイプラインでの役割

1. **ファイルフィルタリング**: 閾値以下のファイルをスキップ
2. **優先度付け**: 高リスクファイルを優先的にLLM解析
3. **コンテキスト強化**: マッチした領域をLLMに提供
4. **脆弱性タイプのヒント**: パターンカテゴリに基づく分類

## パフォーマンス最適化

- **コンパイル時最適化**: 起動時にパターンをコンパイル
- **並列処理**: 複数ファイルの同時解析
- **早期終了**: 高信頼度マッチで処理を停止
- **キャッシュ**: コンパイル済みパターンの再利用

## 設定とカスタマイゼーション

パターンは`src/patterns/`ディレクトリで言語別に管理され、以下の構造で定義されます：

```yaml
# 例: src/patterns/python.yml
principals:
  - pattern: "\\brequests\\."
    description: "HTTP requests library"
    attack_vector:
      - "T1071"  # Application Layer Protocol
      - "T1090"  # Proxy

actions:
  - pattern: "\\bhtml\\.escape\\s*\\("
    description: "HTML escaping action"
    attack_vector:
      - "T1055"  # Process Injection
      - "T1106"  # Native API

resources:
  - pattern: "\\bopen\\s*\\("
    description: "File operations resource"
    attack_vector:
      - "T1083"  # File and Directory Discovery
      - "T1005"  # Data from Local System
```

サポートされる言語ファイル:
- `python.yml`, `javascript.yml`, `typescript.yml`
- `rust.yml`, `java.yml`, `go.yml`, `ruby.yml`
- `c.yml`, `cpp.yml`
- `terraform.yml`, `kubernetes.yml`

## 将来の拡張

- **セマンティック解析**: ASTベースのパターンマッチング
- **機械学習統合**: プロジェクト固有パターンの学習
- **インタラクティブチューニング**: ユーザーフィードバックによる改善

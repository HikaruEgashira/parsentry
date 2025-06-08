# PAR framework（Principal-Action-Resource）

## 概要

PAR（Principal-Action-Resource）frameworkは、security関連操作の3つの主要componentを体系化することで脆弱性を分類する解析modelです：

- **Principal**: 操作を実行する主体（user、service、process）
- **Action**: 実行される操作内容（読み取り、書き込み、実行、認証）
- **Resource**: access対象（file、database、network endpoint、memory）

## 起源と着想

PAR frameworkは既存のsecurity解析概念に基づきながら、より構造化されたアプローチを提供します：

### taint tracking model
従来の静的解析toolはtaint trackingによりdata flowを追跡します：
- **Sources**: 信頼できないdataの流入点
- **Sinks**: dataによるsecurity問題発生点
- **Validation**: sourceとsink間のsanitization・check

PAR frameworkはこのmodelを拡張し：
- **Principal**は従来のsourceとそれを制御するentityを包含
- **Action**は検証・sanitization操作および実際の実行操作を含有
- **Resource**はsinkを一般化し、全種類のtargetとsystemリソースを包含

### Cedar言語の影響
Amazon Cedarの認可言語は以下に基づがpolicy frameworkを提供します：
- **Principal**: requestを実行するentity
- **Action**: requestされる操作
- **Resource**: 操作対象

ParsentryのPAR frameworkはこれらの概念を脆弱性解析に適用：
- Cedarは許可操作の認可決定に重点
- PARは潜在的脆弱性の特定を目的とするsecurity解析に重点
- 双方ともsecurity関係性の体系的推論手法を提供

## PARフレームワークの意義

これらの基礎概念に基づき、PARフレームワークは従来の脆弱性スキャナーより体系的なアプローチを提供：

1. **包括的カバレッジ**: セキュリティ操作の全側面を確実に解析
2. **コンテキスト認識**: アクター、操作、ターゲット間の関係性を考慮
3. **スケーラブル分類**: 異なる言語・技術スタック間で動作
4. **リスク評価**: プリンシパル権限とリソース感度に基づく優先度付けを実現

## PARコンポーネント詳細

### Principal（主体）
操作を開始または制御するエンティティ：
- **ユーザーアカウント**（認証済み、匿名、特権ユーザー）
- **サービスアカウント**（データベースユーザー、APIキー、サービスプリンシパル）
- **プロセス**（システムプロセス、アプリケーションスレッド）
- **外部システム**（サードパーティAPI、マイクロサービス）

### Action（操作）
実行される操作：
- **データ操作**（読み取り、書き込み、更新、削除）
- **認証**（ログイン、ログアウト、トークン生成）
- **認可**（権限チェック、ロール割り当て）
- **システム操作**（ファイルI/O、ネットワーク呼び出し、プロセス実行）
- **暗号化操作**（暗号化、署名、ハッシュ化）

### Resource（リソース）
操作対象：
- **データストア**（データベース、ファイル、メモリ）
- **ネットワークリソース**（エンドポイント、プロトコル、証明書）
- **システムリソース**（プロセス、サービス、ハードウェア）
- **ビジネスロジック**（ワークフロー、トランザクション、状態）

## PARベース脆弱性解析

ParsentryはPARフレームワークを使用してセキュリティ問題を体系的に特定します：

### 1. Principal解析
- コード内の全アクターを特定
- 権限レベルと信頼境界を解析
- 権限昇格機会を検出
- 認証・認可フローをマッピング

### 2. Action解析
- セキュリティ関連操作を全てカタログ化
- 安全でない関数や非推奨関数を特定
- 入力検証・サニタイゼーションを解析
- ビジネスロジック欠陥を検出

### 3. Resource解析
- 全アクセスリソースをマッピング
- 機密データフローを特定
- アクセス制御メカニズムを解析
- リソース枯渇脆弱性を検出

### 4. PAR関係性解析
- コンポーネント間の相互作用を検証
- 信頼境界違反を特定
- 認可バイパスを検出
- データフローセキュリティを解析

## 例：脆弱性解析

以下の脆弱なコードを考えます：
```javascript
function validateEmail(email) {
    // 脆弱な正規表現 - アンカーがない
    const emailRegex = /[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}/;
    return emailRegex.test(email);
}

function processUser(userData) {
    if (validateEmail(userData.email)) {
        // バイパス例: "malicious<script>alert('xss')</script>user@example.com"
        document.getElementById('welcome').innerHTML = `Welcome ${userData.email}`;
    }
}
```

## 解析フレームワーク比較

### 従来の汚染追跡：
- **Source**: `userData.email`（信頼できない入力）
- **Sink**: `innerHTML`（DOMインジェクションポイント）
- **Validation**: ありだが不十分 - 正規表現バイパス可能

### Cedarスタイルポリシー解析：
- **Principal**: Webアプリケーションサービス
- **Action**: DOM操作
- **Resource**: ユーザーインターフェース
- **Policy**: 「アプリケーションはUI更新可能」（認可されているが安全でない実装）

### PARフレームワーク解析：
- **Principal**: Webインターフェース経由のユーザー入力（信頼できないソース）
- **Action**: 欠陥のある正規表現によるメール検証（脆弱な検証実装）
  - 正規表現に`^`と`$`アンカーがない
  - 悪意のある文字列内の有効なメールを許可
  - 偽の信頼境界を作成
- **Resource**: `innerHTML`経由のDOM（XSS実行コンテキスト）

**PARの利点**: 信頼関係（ユーザー入力 vs. アプリケーション）を捕捉し、セキュリティアクションの品質を解析することで、従来のアプローチ単体より包括的な解析を提供。

## PAR Patterns Across Contexts

### JavaScript/Python/Ruby Libraries and Applications
**Library Level:**
- **P**: Function arguments, configuration objects, imported modules
- **A**: Data processing, validation functions, crypto operations
- **R**: Return values, file system, network endpoints

**Web Application Level:**
- **P**: User sessions, API clients, service accounts
- **A**: HTTP requests, database queries, authentication
- **R**: User data, configuration files, external APIs

**PAR Advantage**: Unlike traditional scanners that focus on user input flows, PAR can analyze security issues in library code, utility functions, and internal APIs regardless of the application context.

### Infrastructure as Code (Terraform)
- **P**: Cloud services, deployment pipelines, operators
- **A**: Resource provisioning, permission grants, network configuration
- **R**: Cloud resources, security groups, IAM policies

### System Programming (C/C++/Rust)
- **P**: Processes, threads, system users
- **A**: Memory allocation, file I/O, system calls
- **R**: Memory regions, file systems, hardware interfaces

### Framework Agnostic Analysis
Traditional vulnerability scanners typically require:
- Web application context (HTTP requests, form inputs)
- User input as vulnerability sources
- Application-specific sinks (responses, database writes)

PAR framework enables analysis of:
- **Standalone libraries** without web context
- **Internal APIs and utilities** 
- **Framework-independent code**
- **Microservices and serverless functions**
- **CLI tools and system utilities**

This versatility allows PAR-based analysis to identify vulnerabilities in reusable components that might be missed by application-focused scanners.

## Integration with LLM Analysis

The PAR framework guides LLM prompts to ensure comprehensive analysis:

1. **Structured Prompts**: Each analysis request includes PAR context
2. **Systematic Coverage**: Ensures all three dimensions are evaluated
3. **Consistent Classification**: Provides uniform vulnerability categorization
4. **Risk Scoring**: Enables PAR-based severity assessment

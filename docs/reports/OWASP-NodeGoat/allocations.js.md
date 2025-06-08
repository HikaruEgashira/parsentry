# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **req.params.userId**: Untrusted
  - Context: HTTPリクエストURLパラメータ
  - Risk Factors: 改ざん可能, 未検証
- **req.query.threshold**: Untrusted
  - Context: HTTPクエリパラメータ
  - Risk Factors: 改ざん可能

### Actions (セキュリティ制御)

- **userId取得(req.params.userId)**: Missing
  - Function: 認証・認可
  - Weaknesses: パラメータから直接userIdを取得し認可チェックなし
  - Bypass Vectors: URLパラメータを変更して他ユーザのデータにアクセス
- **データ取得(allocationsDAO.getByUserIdAndThreshold)**: Adequate
  - Function: データベース読み取り
  - Weaknesses: 
  - Bypass Vectors: 

### Resources (操作対象)

- **allocationsデータ**: High
  - Operation: データベース読み取り
  - Protection: 
- **レスポンスレンダリング**: Medium
  - Operation: HTMLレンダリング
  - Protection: 

### Policy Violations

#### C4: IDOR: ユーザ入力による直接オブジェクト参照

- **Path**: AllocationsHandler.displayAllocations -> req.params.userId
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

本コードでは、HTTPリクエストのURLパラメータ(req.params.userId)を用いてデータベースからユーザ毎の割当データを取得しており、認可チェックが実装されていません。その結果、他ユーザのIDを指定することで任意のユーザデータを閲覧可能となるIDOR(Insecure Direct Object Reference)脆弱性があります。

## PoC（概念実証コード）

```text
概念実証:
ユーザ123としてログイン後、以下のリクエストを実行
GET /allocations/456?threshold=10 HTTP/1.1
Host: example.com
Cookie: session=valid_session_for_user_123

レスポンスにユーザ456の割当データが含まれるため、他ユーザデータを不正取得可能
```

## 修復ガイダンス

### displayAllocations

- **Required**: 認可チェックの実装
- **Guidance**: req.session.userIdを用いてセッションユーザIDを取得し、req.params.userIdと一致する場合のみDBクエリを実行する。あるいは、パスパラメータからのuserId取得を廃止しセッションユーザIDのみを利用する。
- **Priority**: high

## 解析ノート

1. Principal: req.params.userId (URLパラメータから取得、信頼できない)\n2. Action: userIdをパスパラメータから直接取得し、認可チェックを行っていない (implementation_quality=missing)\n3. Resource: allocationsDAO.getByUserIdAndThresholdでDBから読み取り (sensitivity=high)\n4. 脆弱性: 他ユーザIDを指定するだけで任意のユーザデータ閲覧(IDOR)\n5. remediation: セッションID経由(userId)での認可チェック追加 or パスパラメータ排除


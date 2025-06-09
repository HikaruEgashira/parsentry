# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **HTTP リクエスト発行者**: Untrusted
  - Context: req.body
  - Risk Factors: ユーザー制御入力あり

### Actions (セキュリティ制御)

- **benefitsDAO.updateBenefits**: Missing
  - Function: authorization
  - Weaknesses: 認可チェック欠如
  - Bypass Vectors: 直接 HTTP リクエスト送信
- **benefitStartDate バリデーション**: Missing
  - Function: input_validation
  - Weaknesses: 入力バリデーション欠如
  - Bypass Vectors: 直接 HTTP リクエスト送信

### Resources (操作対象)

- **User benefits record**: High
  - Operation: update
  - Protection: 

### Policy Violations

#### OWASP-A01-IDOR: 非管理者ユーザーによる任意レコード更新を防ぐ認可チェックがない

- **Path**: BenefitsHandler.updateBenefits -> benefitsDAO.updateBenefits
- **Severity**: high
- **Confidence**: 0.95

## 詳細解析

コードでは、HTTPリクエストの req.body.userId と req.body.benefitStartDate を信頼せず、認可チェックや入力バリデーションを行わずに直接 DAO の updateBenefits を呼び出しているため、非管理者ユーザーでも任意のユーザーの福利厚生開始日を更新できる IDOR 脆弱性が存在します。

## PoC（概念実証コード）

```text
curl -X POST https://<ホスト>/updateBenefits -H 'Content-Type: application/json' -d '{"userId":"victimUserId","benefitStartDate":"2025-01-01"}'
```

## 修復ガイダンス

### BenefitsHandler.updateBenefits

- **Required**: 管理者チェックの実装
- **Guidance**: リクエストに認可済みの管理者権限を持つユーザー情報（例：req.user.isAdmin）を導入し、false の場合は更新処理を拒否する
- **Priority**: high

### 入力バリデーション層

- **Required**: benefitStartDate の形式チェック
- **Guidance**: 日付フォーマット（YYYY-MM-DD など）をバリデーションライブラリで検証し不正な入力は拒否する
- **Priority**: medium

## 解析ノート

- Principal: req.body は信頼できない入力
- Resource: DB のユーザー福利厚生開始日更新は高感度
- Action: 認可・バリデーションなし -> IDOR
- 改善: 認可チェックと日付バリデーションを追加


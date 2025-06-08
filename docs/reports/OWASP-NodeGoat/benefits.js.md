# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **req.body.userId**: Untrusted
  - Context: HTTPリクエストボディ
  - Risk Factors: 改竄可能なパラメータ, 不正アクセスの起点
- **req.body.benefitStartDate**: Untrusted
  - Context: HTTPリクエストボディ
  - Risk Factors: 不正な日付フォーマット, 攻撃用ペイロード挿入

### Actions (セキュリティ制御)

- **updateBenefits(req.body)**: Missing
  - Function: Authorization and Input Validation
  - Weaknesses: 認可チェック欠如, 入力値バリデーション欠如
  - Bypass Vectors: 

### Resources (操作対象)

- **benefitsDAO.updateBenefits**: Medium
  - Operation: database update
  - Protection: 

### Policy Violations

#### AUTH-001: 管理者権限チェックが未実装で、未認可ユーザーがリソースを更新可能

- **Path**: BenefitsHandler.updateBenefits → benefitsDAO.updateBenefits
- **Severity**: high
- **Confidence**: 0.90

#### INPUT-002: userIdおよびbenefitStartDateの入力バリデーションが未実装

- **Path**: BenefitsHandler.updateBenefits
- **Severity**: medium
- **Confidence**: 0.70

## 詳細解析

このコードでは、管理者権限のチェックや入力値のバリデーションが未実装のまま、HTTPリクエストのbodyから受け取ったuserIdとbenefitStartDateを直接データベース更新API（benefitsDAO.updateBenefits）に渡しています。その結果、認証済み・管理者でない攻撃者でも任意のユーザーIDの開始日を変更できるIDOR（Insecure Direct Object Reference）が発生し、権限昇格やデータ改竄につながる可能性があります。

## PoC（概念実証コード）

```text
// 攻撃者が管理者権限なしで任意更新を実行
const request = require('request');
request.post({
  url: 'http://example.com/benefits/update',
  form: { userId: '1', benefitStartDate: '2000-01-01' }
}, (err, res, body) => console.log(body));
```

## 修復ガイダンス

### BenefitsHandler.updateBenefits

- **Required**: 管理者権限チェックと入力バリデーションの実装
- **Guidance**: req.user.isAdmin を確認し、falseなら 403 を返す。また userId は数値チェック、benefitStartDate は日付フォーマット検証を行う。
- **Priority**: high

### benefits-dao

- **Required**: パラメータバインディングとプリペアドステートメントの利用
- **Guidance**: SQLクエリを文字列連結せず、Prismaやライブラリのプリペアドステートメント機能で実装する。
- **Priority**: medium

## 解析ノート

1. req.body.userId, benefitStartDate はHTTPリクエスト由来で untrusted
2. updateBenefits 内で isAdmin チェックがない → 認可欠如
3. 入力値検証もなし → 不正データ挿入可能
4. DAO への直接渡しで IDOR を誘発
5. ポリシー違反: AUTH-001, INPUT-002
6. remediation: isAdmin チェック, バリデーション, プリペアドステートメント導入


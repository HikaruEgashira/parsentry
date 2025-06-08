# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

### Principals (データ源)

- **event['policy_names']**: Untrusted
  - Context: Lambdaイベントペイロード
  - Risk Factors: 未サニタイズの外部入力, 構造化クエリの注入可能性
- **event['user_name']**: Untrusted
  - Context: Lambdaイベントペイロード
  - Risk Factors: 未検証のユーザー名入力

### Actions (セキュリティ制御)

- **db.queryによるSQL実行**: Missing
  - Function: データベースからのポリシー文書取得
  - Weaknesses: SQLインジェクション
  - Bypass Vectors: セミコロンによるステートメント区切りとコメントアウトによる任意SQL実行

### Resources (操作対象)

- **my_database.db - policiesテーブル**: High
  - Operation: SQLクエリ実行 (SELECT)
  - Protection: 
- **IAM put_user_policy API**: Critical
  - Operation: IAMユーザーポリシーの付与
  - Protection: AWS SDK認証

### Policy Violations

#### SQLI-001: 未検証の外部入力を直接SQLクエリに挿入

- **Path**: main.py:20 statement = f"select policy_document from policies where policy_name='{policy}'"
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

本コードでは、外部から渡された `event['policy_names']` の値を検証・サニタイズせずにSQL文へ直接埋め込んでおり、セミコロン区切りで複数ステートメントを実行できる仕組みを許している。その結果、SQLインジェクションが成立し、任意のポリシー文書を取得して `put_user_policy` を通じた権限昇格が可能となる。

## PoC（概念実証コード）

```text
payload = {
    "policy_names": [
        "CustomPolicy1'; SELECT '{\"Version\":\"2012-10-17\",\"Statement\":[{\"Effect\":\"Allow\",\"Action\":\"iam:*\",\"Resource\":\"*\"}]} ' as policy_document--"
    ],
    "user_name": "attacker"
}
handler(payload, None)
```

## 修復ガイダンス

### main.py

- **Required**: SQLクエリのパラメータ化またはホワイトリストによる入力検証
- **Guidance**: SQLiteUtilsのパラメータバインディング機能を利用し、event['policy_names'] は事前定義された有効リストと照合することでSQLインジェクションを防止する。
- **Priority**: high

## 解析ノート

外部入力を f-string で直接 SQL 文に埋め込んでおり、さらにステートメントを分割して複数文が実行可能なため、SQLインジェクションの危険性が高い。取得した policy_document がそのまま IAM ポリシーとして適用されるフローのため、悪意あるポリシー挿入による権限昇格が成立する。妥当な対策はクエリのパラメータ化と入力のホワイトリスト検証。


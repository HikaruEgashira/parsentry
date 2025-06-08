# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 95**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

## 詳細解析

The `handler` function directly interpolates the untrusted `policy` value into an SQL string without any sanitization or parameterization, then splits the SQL by semicolons to execute multiple statements. This allows an attacker to terminate the intended query, inject arbitrary SQL (e.g. a fake policy document), and escalate privileges by calling `iam_client.put_user_policy` with attacker-controlled policy JSON.

## PoC（概念実証コード）

```text
payload = {
    "policy_names": [
        "CustomPolicy1' ; select '{\"Version\":\"2012-10-17\",\"Statement\":[{\"Sid\":\"X\",\"Effect\":\"Allow\",\"Action\":\"iam:*\",\"Resource\":\"*\"}]}' as policy_document--"
    ],
    "user_name": "evil_user"
}
# Calling handler(payload, None) will attach a full-privilege policy document to evil_user
```

## 修復ガイダンス

### handler

- **Required**: Use parameterized queries or ORM APIs
- **Guidance**: Replace f-string SQL construction with prepared statements (e.g. `db.execute("SELECT policy_document FROM policies WHERE policy_name = ?", (policy,))`). Do not split queries on semicolons.
- **Priority**: high

## 解析ノート

1. Identify untrusted input: `event['policy_names']`. 2. Spot dynamic SQL via f-string: `select ... where policy_name='{policy}'`. 3. Note split on `;` enables multi-statement injection. 4. Confirm calls to `db.query` execute injected statements. 5. Conclude SQL injection allows arbitrary policy injection and privilege escalation.


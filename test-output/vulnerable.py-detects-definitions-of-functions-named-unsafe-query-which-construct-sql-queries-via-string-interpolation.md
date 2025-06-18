# Security Analysis: vulnerable.py - Detects definitions of functions named 'unsafe_query', which construct SQL queries via string interpolation

## ファイル情報

- **ファイルパス**: `test-sample/vulnerable.py`
- **検出パターン**: Detects definitions of functions named 'unsafe_query', which construct SQL queries via string interpolation

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

### Principals (データ源)

- **user_input**: Untrusted
  - Context: input()
  - Risk Factors: unsanitized_input

### Actions (セキュリティ制御)

- **string_interpolation**: Insufficient
  - Function: f-string SQL 組み立て
  - Weaknesses: Improper Input Validation
  - Bypass Vectors: SQL Injection via crafted input

### Resources (操作対象)

- **execute_query**: High
  - Operation: database_query
  - Protection: 

### Policy Violations

#### SQLI_001: 信頼できない入力が検証なしで直接SQLクエリに埋め込まれているため、SQLインジェクションが発生する可能性があります。 (Pattern: Detects definitions of functions named 'unsafe_query', which construct SQL queries via string interpolation)

- **Path**: unsafe_query -> execute_query
- **Severity**: high
- **Confidence**: 0.90

## マッチしたソースコード

```code
unsafe_query
```

## 完全なソースコード

```code
#!/usr/bin/env python3

def unsafe_query(user_id):
    """Vulnerable SQL injection example"""
    query = f"SELECT * FROM users WHERE id = {user_id}"
    return execute_query(query)

def execute_query(sql):
    """Mock database execution"""
    print(f"Executing: {sql}")
    return []

# Test vulnerable function
if __name__ == "__main__":
    user_input = input("Enter user ID: ")
    result = unsafe_query(user_input)
    print(result)
```

## 詳細解析

unsafe_query関数はf文字列を用いてユーザー入力を直接SQLクエリに埋め込んでおり、入力の検証・サニタイズが行われていないため、SQLインジェクションの脆弱性があります。

## PoC（概念実証コード）

```text
ユーザー入力に「1 OR 1=1」を渡すと、全ユーザー情報が返される可能性があります。例: input: 1 OR 1=1
```

## 修復ガイダンス

### unsafe_query

- **Required**: パラメータ化クエリを使用する
- **Guidance**: f文字列ではなく、プレースホルダーとパラメータバインディングを使用してSQLインジェクションを防止してください。
- **Priority**: high

## 解析ノート

対象パターン: unsafe_queryによるf文字列でのSQL組み立て


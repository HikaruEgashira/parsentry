# Security Analysis: vulnerable.py - Detects definitions of functions named 'execute_query', which execute raw SQL statements

## ファイル情報

- **ファイルパス**: `test-sample/vulnerable.py`
- **検出パターン**: Detects definitions of functions named 'execute_query', which execute raw SQL statements

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

### Principals (データ源)

- **stdinからのユーザー入力**: Untrusted
  - Context: input()関数
  - Risk Factors: 未検証の外部入力

### Actions (セキュリティ制御)

- **unsafe_query**: Missing
  - Function: SQL文字列補間
  - Weaknesses: CWE-89
  - Bypass Vectors: 

### Resources (操作対象)

- **execute_query**: High
  - Operation: SQL実行
  - Protection: 

### Policy Violations

#### CWE-89: ユーザー入力が未検証のままSQL文に埋め込まれている (Pattern: Detects definitions of functions named 'execute_query', which execute raw SQL statements)

- **Path**: unsafe_query -> execute_query
- **Severity**: high
- **Confidence**: 0.80

## マッチしたソースコード

```code
execute_query
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

execute_query関数がユーザー入力を含む生のSQL文をそのまま実行しており、SQLインジェクションが発生する可能性があります。

## PoC（概念実証コード）

```text
例えば、ユーザーIDとして「1 OR 1=1」を入力すると、全ユーザーの情報が取得されます。
```

## 修復ガイダンス

### unsafe_query

- **Required**: パラメータ化されたクエリを使用
- **Guidance**: 文字列補間ではなく、プリペアドステートメントまたはORMのパラメータバインディングを使用してください。
- **Priority**: 高


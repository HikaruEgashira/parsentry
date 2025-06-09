# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

### Principals (データ源)

- **threshold**: Untrusted
  - Context: HTTP request parameter
  - Risk Factors: user-controlled, unsanitized
- **userId**: Untrusted
  - Context: HTTP request parameter
  - Risk Factors: user-controlled

### Actions (セキュリティ制御)

- **searchCriteria**: Missing
  - Function: input validation
  - Weaknesses: NoSQL injection
  - Bypass Vectors: 0';while(true){}, 1'; return 1=='1

### Resources (操作対象)

- **allocationsCol.find**: Medium
  - Operation: database query
  - Protection: 
- **allocationsCol.update**: Medium
  - Operation: database update
  - Protection: 

### Policy Violations

#### CWE-743: Improper neutralization of input in MongoDB $where – NoSQL injection

- **Path**: AllocationsDAO.getByUserIdAndThreshold -> searchCriteria -> $where concatenation with threshold
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

`getByUserIdAndThreshold` uses the user-supplied `threshold` directly in a MongoDB `$where` clause via string concatenation, enabling NoSQL injection. There is no input validation or sanitization for `threshold`, so attackers can inject JavaScript code into the query. Other inputs (userId, stocks, funds, bonds) are parsed or passed without strict checks but the immediate critical issue is the unsanitized threshold in `$where`.

## PoC（概念実証コード）

```text
// Proof of Concept: NoSQL Injection via threshold parameter
// Request: GET /allocations?userId=1&threshold=0'; while(true){} // server hangs executing injected loop

// Or injection bypass:
// GET /allocations?userId=1&threshold=1'; return this; var a='
```

## 修復ガイダンス

### AllocationsDAO.getByUserIdAndThreshold

- **Required**: Implement strict input validation and avoid $where string concatenation
- **Guidance**: Parse threshold to integer (e.g., const parsedThreshold = parseInt(threshold, 10)); enforce range checks; replace $where with parameterized query: allocationsCol.find({ userId: parsedUserId, stocks: { $gt: parsedThreshold } })
- **Priority**: high

## 解析ノート

Identified NoSQL injection in getByUserIdAndThreshold -> searchCriteria using $where and user-supplied threshold without validation. Proposed remediation using parameterized query and integer parsing.


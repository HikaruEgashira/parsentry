# Security Analysis: messages_controller.rb - Command line arguments

## ファイル情報

- **ファイルパス**: `repo/app/controllers/messages_controller.rb`
- **検出パターン**: Command line arguments

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 95**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

## マッチしたソースコード

```code
ApplicationController
```

## 詳細解析

## IDOR Vulnerability: show method

**Principal**: Authenticated user via `params[:id]`
**Action**: Direct object reference without authorization check
**Resource**: Message record in database

The `show` method retrieves a message using only the message ID without verifying that the current user owns or has permission to view that message.

**Impact**: An attacker can view any message in the system by manipulating the `id` parameter.

**Attack Path**:
1. Attacker authenticates as User A
2. Attacker requests `/messages/999` (a message belonging to User B)
3. Query executes: `Message.where(id: 999).first`
4. Without ownership check, User B's message is returned to attacker

**CWE**: CWE-639 (Authorization Through User-Controlled Key), CWE-284 (Improper Access Control)
**OWASP**: A01:2021 - Broken Access Control


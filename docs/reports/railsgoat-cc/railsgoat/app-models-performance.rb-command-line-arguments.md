# Security Analysis: performance.rb - Command line arguments

## ファイル情報

- **ファイルパス**: `repo/app/models/performance.rb`
- **検出パターン**: Command line arguments

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 75**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

## マッチしたソースコード

```code
User
```

## 詳細解析

## Insecure Direct Object Reference (IDOR)

### Vulnerability Description
The `reviewer_name` method retrieves User objects based on the `self.reviewer` field without verifying that the requesting user has authorization to access that user's information.

### Attack Path
1. Attacker accesses a Performance record they control
2. Attacker manipulates the `reviewer` field to point to any user ID
3. Method returns the full name of the target user
4. Attacker can enumerate all user full names in the system

### Code Location
```ruby
def reviewer_name
  u = User.find_by_id(self.reviewer)  # No authorization check
  u.full_name if u.respond_to?("fullname")
end
```

### Remediation
Add authorization checks before retrieving user information:
- Verify the requesting user has permission to view this user's details
- Implement role-based access control
- Use authorization gems like Pundit or CanCanCan


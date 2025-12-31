# Security Analysis: user_mailer.rb - Command line arguments

## ファイル情報

- **ファイルパス**: `repo/app/mailers/user_mailer.rb`
- **検出パターン**: Command line arguments

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 72**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

## マッチしたソースコード

```code
Base
```

## 詳細解析

The `token` parameter is directly interpolated into the URL string at line 7 using `#{token}`. This can lead to XSS if the URL is rendered in HTML context within the email or elsewhere without proper escaping.


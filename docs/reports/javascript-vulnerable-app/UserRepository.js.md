# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SQLI`

## 解析結果

このUserRepository.jsには、全てのDBクエリ構築箇所で入力パラメータを直接文字列連結・埋め込みしており、ユーザー入力によるSQLインジェクションが多重に発生しています。特にID／username／emailなどの単一パラメータを埋め込むメソッド（findById, findByUsername, findByEmail）、動的フィルタ・ソート・制限を構築するfindAll、可変フィールドを更新するupdate、認証用authenticate、汎用executeQueryなどが脆弱です。適切なプリペアドステートメントやパラメータ化クエリを利用せず、重大な認証バイパスや任意SQL実行を許します。

## PoC（概念実証コード）

```text
// 認証バイパス例
const repo = new UserRepository();
repo.authenticate("' OR '1'='1", "any").then(user => console.log(user));

// findByIdで全件取得例
repo.findById('1 OR 1=1').then(rows => console.log(rows));

// findAllでDROP TABLE実行例
repo.findAll({orderBy: "id; DROP TABLE users;--"});
```

## 関連コードコンテキスト

### 関数名: findById
- 理由: IDパラメータを直接埋め込んでおり、例: id = 1 OR 1=1 による全件取得が可能です。
- パス: example/javascript-vulnerable-app/infrastructure/database/UserRepository.js:12
```rust
const query = `SELECT * FROM users WHERE id = ${id}`;
```

### 関数名: findByUsername
- 理由: usernameをエスケープせずに文字列連結し、' OR '1'='1 などで認証バイパスできます。
- パス: example/javascript-vulnerable-app/infrastructure/database/UserRepository.js:22
```rust
const query = `SELECT * FROM users WHERE username = '${username}'`;
```

### 関数名: findAll
- 理由: orderBy句を直接挿入し、DROP TABLEなど任意クエリを混入できます。
- パス: example/javascript-vulnerable-app/infrastructure/database/UserRepository.js:46
```rust
if (filters.orderBy) { query += ` ORDER BY ${filters.orderBy}`; }
```

### 関数名: update
- 理由: 可変更新フィールドとIDを直接連結しており、update.idやdataフィールドでSQLインジェクション可能です。
- パス: example/javascript-vulnerable-app/infrastructure/database/UserRepository.js:71
```rust
const query = `UPDATE users SET ${updates.join(', ')} WHERE id = ${id}`;
```

### 関数名: authenticate
- 理由: 認証クエリを直接文字列連結し、' OR '1'='1による認証バイパスを許します。
- パス: example/javascript-vulnerable-app/infrastructure/database/UserRepository.js:100
```rust
const query = `SELECT * FROM users WHERE username = '${username}' AND password = '${password}'`;
```

### 関数名: executeQuery
- 理由: 任意のSQLを引数として実行可能で、管理画面などから悪意あるクエリ実行を許す恐れがあります。
- パス: example/javascript-vulnerable-app/infrastructure/database/UserRepository.js:131
```rust
this.db.all(query, params, (err, rows) => { ... });
```

## 解析ノート

コードを見たところ、DBアクセスは全てsqlite3でプリペアドステートメントを使わず、入力を直接SQL文字列に埋め込んでいます。findById/findByUsername/findByEmailは単純埋め込み、findAllはフィルタとorderBy/limitも埋め込み、create/update/delete/authenticate/searchも同様。executeQueryは任意SQL実行をラップしており、管理者権限などの制限がない場合は直接DB操作を乗っ取れます。これら全てに対してパラメータバインディングや入力検証がなく、典型的なSQLインジェクション脆弱性です。


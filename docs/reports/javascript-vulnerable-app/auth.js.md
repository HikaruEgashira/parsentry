# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `SQLI`
- `IDOR`

## 解析結果

認証処理において「alg: 'none'」を許可しているため、署名検証をバイパスして任意のpayload（user_id）を注入できます。さらに、SQLクエリを文字列連結で組み立てており、user_idがそのままクエリに埋め込まれるため、SQLインジェクションが成立します。また、x-role-overrideヘッダー等により権限を動的に昇格できるため、IDOR的な権限エスカレーションも可能です。

## PoC（概念実証コード）

```text
1. JWTヘッダーで{"alg":"none"}、ペイロードに{"user_id":"1 OR 1=1"}を設定し、署名を空にしたトークンを生成
2. HTTPヘッダーにAuthorization: Bearer <改ざんトークン>を設定してエンドポイントにアクセス
3. verify処理で検証がスキップされ、decoded.user_idに"1 OR 1=1"が入り、SQLクエリに注入されて全ユーザー情報を取得可能

-- x-role-overrideヘッダー利用例 --
1. リクエストヘッダーに x-role-override: admin を追加
2. requireRoleによりユーザー権限がadminに昇格され、保護リソースへのアクセスが可能
```

## 関連コードコンテキスト

### 関数名: authenticateToken
- 理由: 署名検証で'none'アルゴリズムを許可しており、改ざんトークンで認証バイパスが可能
- パス: repo/middleware/auth.js
```rust
const decoded = jwt.verify(token, JWT_SECRET, { algorithms: ['HS256', 'none'] });
```

### 関数名: authenticateToken
- 理由: user_idが直接埋め込まれた動的SQL。攻撃者制御下のpayloadでSQLインジェクションを引き起こす
- パス: repo/middleware/auth.js
```rust
const query = `SELECT * FROM users WHERE id = ${decoded.user_id}`;
```

### 関数名: requireRole
- 理由: x-role-overrideヘッダーで任意のロールを設定でき、権限チェックをバイパス可能
- パス: repo/middleware/auth.js
```rust
if (roleOverride) { req.user.role = roleOverride; return next(); }
```

## 解析ノート

認証処理、SQLクエリ組み立て、権限チェックのバイパスポイントを特定
・jwt.verifyのalg none許可 => 攻撃者が任意payload埋め込み
・payload.user_id -> 動的SQLに直接埋め込み => SQLインジェクション
・requireRoleでx-role-overrideにより権限昇格 => IDOR的利用
評価: 実用的な攻撃手法として高リスク


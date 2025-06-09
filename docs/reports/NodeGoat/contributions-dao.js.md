# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **userIdパラメータ**: Untrusted
  - Context: クライアントパラメータ
  - Risk Factors: 信頼できない入力

### Actions (セキュリティ制御)

- **IDOR-001**: Missing
  - Function: アクセス制御
  - Weaknesses: 認可チェックの欠如, セッション・ユーザーIDとの照合なし
  - Bypass Vectors: パラメータ改ざんによる任意userId指定, 水平権限エスカレーション

### Resources (操作対象)

- **contributionsDB**: High
  - Operation: read/write
  - Protection: 

## 詳細解析

本コードでは、userIdパラメータをクライアントから直接受け取り、そのままDBクエリの条件として使用しています。認可チェックやセッションに紐づく認証情報との照合が一切行われておらず、任意のuserIdを指定することで他ユーザーの資産情報（contributions）を閲覧・更新できてしまいます。したがって、Insecure Direct Object Reference (IDOR) の脆弱性が存在します。

PARモデルによる評価:
1. Principal（主体）: ユーザー入力のuserId（untrusted）
2. Resource（リソース）: contributionsDBコレクション内の他ユーザーのcontributionsドキュメント（sensitivity: high）
3. Action（操作）: update()、getByUserId() によるドキュメントの読み書き
4. 防御策: 入力検証や認可チェックが実装されておらず、容易にバイパス可能
5. ポリシー違反: untrustedなPrincipalが適切な認可なしで高感度Resourceへの直接アクセスを行える

## PoC（概念実証コード）

```text
// 攻撃者が他ユーザーのIDを指定して情報を更新・取得できるサンプル
var dao = new ContributionsDAO(db);
// victimUserId は攻撃対象のユーザーID
var victimUserId = 12345;
da o.update(victimUserId, 0, 0, 0, function(err, res) {
  if (!err) console.log('攻撃成功:', res);
});

dao.getByUserId(victimUserId, function(err, contributions) {
  if (!err) console.log('被害ユーザー情報:', contributions);
});
```

## 修復ガイダンス

### ContributionsDAO

- **Required**: セッションやトークンベースの認証情報とuserIdを照合して認可チェックを実装する
- **Guidance**: 呼び出し元のコンテキストからログインユーザーのIDを取得し、引数のuserIdと比較して一致しない場合はエラーを返す
- **Priority**: high

## 解析ノート

- userId を parseInt しているが、クエリ自体は文字列のまま使用している
- 認可チェック不在で任意の ID を更新・取得可能
- IDOR の典型的パターン
- fix: セッションから userId を取得し、引数より優先させる


# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **userId パラメータ**: Untrusted
  - Context: 関数引数
  - Risk Factors: 改ざん可能, 権限なしで他人のID指定可

### Actions (セキュリティ制御)

- **ContributionsDAO.update**: Missing
  - Function: 認可（Access Control）
  - Weaknesses: 認可チェック欠如, 直接DB操作
  - Bypass Vectors: 任意のuserIdパラメータを指定
- **ContributionsDAO.getByUserId**: Missing
  - Function: 認可（Access Control）
  - Weaknesses: 認可チェック欠如, 直接DBアクセス
  - Bypass Vectors: 任意のuserIdパラメータを指定

### Resources (操作対象)

- **contributions コレクション**: Medium
  - Operation: update/find
  - Protection: 

### Policy Violations

#### AC-USER-OWNERSHIP: ユーザーは自身のリソースのみ操作可能であるべき

- **Path**: ContributionsDAO.update -> contributionsDB.update
- **Severity**: high
- **Confidence**: 0.90

#### AC-USER-OWNERSHIP: ユーザーは自身のリソースのみ閲覧可能であるべき

- **Path**: ContributionsDAO.getByUserId -> contributionsDB.findOne
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

このコードでは、API呼び出し元から渡された任意のuserIdを用いてDBの更新・取得を行っており、呼び出し元の認可チェックが存在しません。そのため、攻撃者が他のユーザーIDを指定することで、他人の拠出額情報を不正に更新・取得できるIDOR（Insecure Direct Object Reference）が発生しています。

## PoC（概念実証コード）

```text
// 攻撃者が他ユーザーID“456”の情報を更新
const dao = new ContributionsDAO(db);
dao.update('456', 100, 100, 100, (err, res) => console.log(res));

// 他ユーザーの情報取得も可能
dao.getByUserId('456', (err, res) => console.log(res));
```

## 修復ガイダンス

### ContributionsDAO.update

- **Required**: 認可チェックの追加
- **Guidance**: 現在ログイン中のユーザーIDとリクエストのuserIdを比較し、一致する場合のみDB操作を許可する。管理者権限のみ他ID操作可とする場合はロールチェックを行う。
- **Priority**: high

### ContributionsDAO.getByUserId

- **Required**: 認可チェックの追加
- **Guidance**: リクエスト元ユーザーIDと取得対象userIdを比較し、一致しない場合は403エラーとするか空オブジェクトを返す。
- **Priority**: high

## 解析ノート

- 機能: ContributionsDAO.update/getByUserIdでuserIdを外部入力としてDB操作
- 問題: 認可チェックなしに任意IDでupdate/findを実行
- 結論: IDOR脆弱性
- 対策: 呼び出し元ユーザーIDと一致確認、ロールベース制御追加
- PoC: dao.update('456',...) で他人情報書き換え可能
- 信頼度: 高（認可欠如は明確）


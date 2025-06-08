# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **外部API呼び出しユーザー**: Untrusted
  - Context: update_setting, is_admin, usernameへの入力
  - Risk Factors: 不正なuser_idパラメータ指定可能

### Actions (セキュリティ制御)

- **update_setting**: Missing
  - Function: ユーザー設定の更新
  - Weaknesses: 認可チェック欠如
  - Bypass Vectors: 
- **is_admin**: Missing
  - Function: 管理者権限判定
  - Weaknesses: 認可チェック欠如
  - Bypass Vectors: 
- **username**: Missing
  - Function: ユーザー名取得
  - Weaknesses: 認可チェック欠如
  - Bypass Vectors: 

### Resources (操作対象)

- **accounts配列**: High
  - Operation: ユーザー情報読み書き
  - Protection: 

### Policy Violations

#### AUTHZ-001: ユーザーは自身のリソースのみ操作可能であるべき

- **Path**: update_setting -> accounts[user_id]
- **Severity**: high
- **Confidence**: 0.90

#### AUTHZ-001: ユーザーは自身のリソースのみ操作可能であるべき

- **Path**: is_admin -> accounts[user_id]
- **Severity**: high
- **Confidence**: 0.90

#### AUTHZ-001: ユーザーは自身のリソースのみ参照可能であるべき

- **Path**: username -> accounts[user_id]
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

本コードでは主に以下の脆弱性が確認されました：
1. create_user_account関数内でuserid_nextを二重にインクリメントしており、accounts配列のインデックスにずれが生じ、一部のslotsがNULLのまま参照される可能性がある（メモリ破壊・クラッシュ）。
2. update_setting／is_admin／usernameの各関数で、呼び出し元のユーザー認可チェックが皆無であり、任意のuser_idを指定することで他ユーザーのデータを読み書きできるIDOR（不適切な参照制御）脆弱性。


## PoC（概念実証コード）

```text
#include <stdio.h>
#include <stdbool.h>
// 任意のuser_id=1 を指定して他ユーザーの設定を上書きする例
int main(){
    // 本来は自身のuser_idのみ変更可能のはず
    if(update_setting(1, "0", "999")){
        printf("IDOR成立: 他ユーザーの設定変更に成功しました\n");
    }
    return 0;
}
```

## 修復ガイダンス

### update_setting

- **Required**: 認可チェックの実装
- **Guidance**: 呼び出し元のユーザーIDと引数 user_id が一致するか、または管理者権限を持つ場合のみ処理を許可する
- **Priority**: high

### is_admin, username

- **Required**: 認可チェックの実装
- **Guidance**: 呼び出し元が対象 user_id の本人または管理者であることを検証した上で情報を返却する
- **Priority**: high

### create_user_account

- **Required**: user_id管理ロジック修正
- **Guidance**: userid_nextを一度だけインクリメントし、accounts[id]=ua; return id; のように実装してオフバイワンを防止する
- **Priority**: medium

## 解析ノート

1. create_user_account内でuserid_nextを++してからaccounts[userid_next]に格納し、さらにreturn時に++しているためインデックスずれ／NULL参照が発生する
2. update_setting/is_admin/usernameにおいて全く認可チェックがなく、任意のuser_idで他ユーザーの情報を読み書き可能
3. これはIDOR（不適切な参照制御）に該当
4. remediationとしてそれぞれ関数に呼び出し元本人確認または管理者判定を追加し、create_user_accountのカウンタ管理を修正する必要あり


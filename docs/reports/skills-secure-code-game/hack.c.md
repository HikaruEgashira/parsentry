# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `AFO`

## PAR Policy Analysis

### Principals (データ源)

- **user入力(keyパラメータ)**: Untrusted
  - Context: update_setting呼び出し時の引数
  - Risk Factors: ユーザー操作可能, 負の数値指定可能

### Actions (セキュリティ制御)

- **update_setting(user1, key, value)**: Insufficient
  - Function: ユーザー設定の更新
  - Weaknesses: インデックス値の境界チェック欠如, 認可チェック欠如
  - Bypass Vectors: 負のインデックス（"-7"）を指定して管理者フラグ領域へ書き込む

### Resources (操作対象)

- **user1のadminフラグ**: Critical
  - Operation: 権限レベル変更
  - Protection: 

### Policy Violations

#### AUTH-01: 未認可のユーザーが保護された管理者フラグを変更できている

- **Path**: update_setting -> 管理者フラグ領域への負のインデックス書き込み
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

このコードでは、update_setting関数に対して「-7」という負のインデックスが入力可能であり、設定配列の境界チェックや認可チェックが存在しないことで、管理者フラグを不正に書き換えられる脆弱性（整数境界外参照による権限昇格）が発生しています。

## PoC（概念実証コード）

```text
#include "code.h"
int main(){int user= create_user_account(false,"pwned"); update_setting(user,"-7","1"); if(is_admin(user)) printf("Escalated to admin!\n");}

```

## 修復ガイダンス

### update_setting

- **Required**: 設定キーの入力値範囲チェックと認可検証を追加
- **Guidance**: keyパラメータが配列の有効範囲内かを検証し、管理者フラグを変更する場合は呼び出し元が管理者であることを確認する
- **Priority**: high

## 解析ノート

・update_settingに"-7"が渡されると負のインデックスでメモリ書き込み可能
・配列境界チェックも認可チェックも存在せず、adminフラグ書き換えを許可
・結果、非管理者が管理者権限を取得する権限昇格脆弱性
・AFO（配列外参照）脆弱性として分類可能
・remediation: 入力バリデーション／認可検証の追加が必要


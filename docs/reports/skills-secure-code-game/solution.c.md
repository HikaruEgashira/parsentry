# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `AFO`

## PAR Policy Analysis

### Principals (データ源)

- **update_setting:index,valueパラメータ**: Untrusted
  - Context: update_setting API呼び出し
  - Risk Factors: 外部ユーザ入力, 数値文字列

### Actions (セキュリティ制御)

- **update_setting:update index**: Bypassed
  - Function: 入力バリデーション
  - Weaknesses: 負の値チェックの欠如, 境界チェック不完全
  - Bypass Vectors: 負のインデックス "-1"

### Resources (操作対象)

- **user_account.setting配列**: High
  - Operation: メモリ書き込み
  - Protection: 境界チェック

### Policy Violations

#### ARR-02: 配列インデックスは0以上かつ上限未満を厳格に検証する必要がある

- **Path**: update_setting -> accounts[user_id]->setting
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

update_setting関数ではindexパラメータに対して負の値の検証が欠如しており、i = -1のときaccounts[user_id]->setting[-1]への書き込みが許される。このオフバイフロー（AFO）を悪用すると、構造体user_accountの直前にあるisAdminフラグを書き換え、一般ユーザーが管理者権限を不正に取得できる。

## PoC（概念実証コード）

```text
#include <stdio.h>
#include <stdbool.h>

int main() {
    int uid = create_user_account(false, "normal");
    // 管理者フラグ書き換え
    update_setting(uid, "-1", "1");
    if (is_admin(uid)) {
        printf("Privilege escalated to admin!\n");
    } else {
        printf("Still normal user.\n");
    }
    return 0;
}

```

## 修復ガイダンス

### update_setting

- **Required**: 配列インデックスに対し負の値チェックを追加
- **Guidance**: strtol後にi>=0およびi<SETTINGS_COUNTを必ず検証し、範囲外ならfalseを返却してください。
- **Priority**: high

## 解析ノート

update_settingのi=strtol(index)後、負の値チェックがなくoff-by-indexが発生。i=-1でaccounts[user_id]->setting[-1]への書き込みが可能。構造体前方のisAdminフラグを改竄でき、特権昇格を許す。


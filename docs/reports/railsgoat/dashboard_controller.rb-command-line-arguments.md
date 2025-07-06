# Security Analysis: dashboard_controller.rb - Command line arguments

## ファイル情報

- **ファイルパス**: `repo/app/controllers/dashboard_controller.rb`
- **検出パターン**: Command line arguments

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `RCE`
- `AFO`

## PAR Policy Analysis

### Principals (データ源)

- **params[:graph]**: Untrusted
  - Context: HTTP request parameter
  - Risk Factors: ユーザー入力, 信頼できない
- **params[:font]**: Untrusted
  - Context: HTTP request parameter
  - Risk Factors: ユーザー入力, 信頼できない

### Resources (操作対象)

- **self.try**: High
  - Operation: method_invocation
  - Protection: 

### Policy Violations

#### PR001: 動的メソッド呼び出しに対する入力検証・ホワイトリストが未実装 (Pattern: Command line arguments)

- **Path**: DashboardController#change_graph: self.try(params[:graph])
- **Severity**: high
- **Confidence**: 0.90

## マッチしたソースコード

```code
ApplicationController
```

## 詳細解析

本コードではchange_graphアクション内でユーザー入力(params[:graph])を検証せずにself.tryメソッドを用いて動的にメソッド呼び出しを行っており、攻撃者が任意のコントローラ内メソッドを実行できる可能性があります。また、homeアクション内のparams[:font]も検証なしにcookieへ設定されており、セキュリティ処理（Action）や入力サニタイズが欠如しています。

## PoC（概念実証コード）

```text
1. 攻撃者は以下のようにリクエストを送信します:
   GET /dashboard/change_graph?graph=redirect_to%3Furl%3Dhttp%3A%2F%2Fevil.com
2. params[:graph]に"redirect_to?url=http://evil.com"が設定され、self.tryが呼ばれることで任意のリダイレクトが発生します。
```

## 修復ガイダンス

### 入力バリデーション

- **Required**: params[:graph]のホワイトリスト制限
- **Guidance**: 許可されたグラフ名(例: "bar_graph","pie_charts")のみを受け入れ、動的メソッド呼び出し(self.try)は削除または安全な分岐に置き換えてください。
- **Priority**: high

### サニタイズ/エスケープ

- **Required**: params[:font]の値検証
- **Guidance**: cookie保存前に許可文字列パターンを適用し、不正な値の設定を防止してください。
- **Priority**: medium

## 解析ノート

- Principal: params[:graph], params[:font] は信頼できないユーザー入力。
- Action: 入力検証・サニタイズ処理が存在しない(空のactions)。
- Resource: self.tryによる動的メソッド呼び出し。
- ポリシー違反: 動的呼び出しに対しホワイトリストが未実装(PR001)。
- 脆弱性: RCE/AFO 相当のリフレクションによる任意メソッド実行。
- 修正: ホワイトリスト制限、動的呼び出し削除、サニタイズ追加。


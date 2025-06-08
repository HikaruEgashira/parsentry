# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SQLI`
- `RCE`
- `SSRF`
- `IDOR`
- `AFO`

## PAR Policy Analysis

### Principals (データ源)

- **GraphQL argument 'filter'**: Untrusted
  - Context: Query.resolve_pastes
  - Risk Factors: SQLインジェクション
- **GraphQL argument 'arg'**: Untrusted
  - Context: Query.resolve_system_debug
  - Risk Factors: コマンドインジェクション
- **GraphQL args 'host','port','path','scheme'**: Untrusted
  - Context: Mutation.ImportPaste
  - Risk Factors: SSRF, コマンドインジェクション
- **GraphQL argument 'id'**: Untrusted
  - Context: Mutation.EditPaste/DeletePaste, Query.resolve_delete_all_pastes
  - Risk Factors: IDOR
- **GraphQL argument 'filename'**: Untrusted
  - Context: Mutation.UploadPaste
  - Risk Factors: ディレクトリトラバーサル,AFO

### Actions (セキュリティ制御)

- **Query.resolve_pastes**: Missing
  - Function: SQLクエリビルド
  - Weaknesses: 直接文字列連結によるSQLクエリ組立て
  - Bypass Vectors: ' OR '1'='1
- **Query.resolve_system_debug**: Missing
  - Function: シェルコマンド実行
  - Weaknesses: ユーザ入力を直接shellに渡す
  - Bypass Vectors: ; ls /, `rm -rf /`
- **Mutation.ImportPaste**: Insufficient
  - Function: 外部リソース取得
  - Weaknesses: 動的URLをcurlに渡す
  - Bypass Vectors: && curl http://malicious
- **Mutation.EditPaste/DeletePaste/delete_all_pastes**: Missing
  - Function: DB更新・削除
  - Weaknesses: 認可チェック欠如
  - Bypass Vectors: idパラメータ任意指定
- **Mutation.UploadPaste**: Missing
  - Function: ファイル保存
  - Weaknesses: ファイルパス検証欠如
  - Bypass Vectors: ../etc/passwd

### Resources (操作対象)

- **Pasteテーブル**: Medium
  - Operation: SELECT/UPDATE/DELETE
  - Protection: 
- **シェルコマンド実行環境**: Critical
  - Operation: EXEC
  - Protection: 
- **外部HTTPリクエスト**: Medium
  - Operation: FETCH
  - Protection: 
- **ファイルシステム**: High
  - Operation: WRITE
  - Protection: 

### Policy Violations

#### POL001: Untrusted input used directly in SQL query (SQL Injection)

- **Path**: Query.resolve_pastes
- **Severity**: high
- **Confidence**: 0.90

#### POL002: Untrusted input used in shell execution (RCE)

- **Path**: Query.resolve_system_debug, Mutation.ImportPaste
- **Severity**: critical
- **Confidence**: 0.90

#### POL003: Missing authorization check on resource manipulation (IDOR)

- **Path**: Mutation.EditPaste, Mutation.DeletePaste, Query.resolve_delete_all_pastes
- **Severity**: high
- **Confidence**: 0.80

#### POL004: Server-Side Request Forgery (SSRF) via uncontrolled curl

- **Path**: Mutation.ImportPaste
- **Severity**: critical
- **Confidence**: 0.80

#### POL005: Unvalidated file path allows arbitrary file write (AFO)

- **Path**: Mutation.UploadPaste
- **Severity**: high
- **Confidence**: 0.70

## 詳細解析

複数のGraphQLリゾルバにおいて、以下の深刻な脆弱性が確認されました。
1. SQLインジェクション (resolve_pastes)：ユーザ入力をフォーマット文字列で直接SQLに渡しているため、任意のSQL実行が可能。
2. リモートコード実行 (system_debug, ImportPaste)：引数をそのままシェルコマンドに渡しており、コマンドインジェクションが可能。
3. SSRF (ImportPaste)：任意のホスト・パスをcurl実行し、内部サービスへリクエスト可能。
4. IDOR/認可欠如 (EditPaste, DeletePaste, delete_all_pastes)：認証・認可チェックが無く、任意のPaste操作が可能。
5. 任意ファイル書き込み (UploadPaste)：ファイルパス検証なしにファイル保存しており、ディレクトリトラバーサルなど任意ファイル上書きが可能。

## PoC（概念実証コード）

```text
# SQLi PoC
query { pastes(filter:"' OR '1'='1") { id title } }

# RCE PoC
query { system_debug(arg:"1; ls /tmp") }

# SSRF PoC
mutation { importPaste(host:"169.254.169.254",port:80,path:"/latest/meta-data/",scheme:"http") { result } }

# IDOR PoC
mutation { deletePaste(id:1) { result } }

# AFO PoC
mutation { uploadPaste(filename:"../etc/passwd",content:"pwned") { result } }
```

## 修復ガイダンス

### Query.resolve_pastes

- **Required**: 常にパラメータ化されたクエリを使用
- **Guidance**: SQLAlchemyのfilter_byやバインド変数を使い、直接文字列連結を排除する
- **Priority**: high

### Query.resolve_system_debug & Mutation.ImportPaste

- **Required**: ユーザ入力のホワイトリスト検証またはエスケープ
- **Guidance**: シェルコマンド呼び出しを廃止し、安全なライブラリを用いるか、最低限厳格なホワイトリストで検証する
- **Priority**: critical

### Mutation.ImportPaste

- **Required**: SSRF防止のためのホスト検証
- **Guidance**: 許可された外部ドメインのホワイトリストチェックを実装する
- **Priority**: critical

### Mutation.EditPaste/DeletePaste/Query.resolve_delete_all_pastes

- **Required**: 認証・認可チェックを導入
- **Guidance**: 操作対象リソースの所有者確認や権限チェックを実装し、他者のリソース操作を防止する
- **Priority**: high

### Mutation.UploadPaste

- **Required**: アップロードパスのサニタイズ
- **Guidance**: ファイル名正規化・禁止文字除去・ディレクトリ制限を行い、パス渡しトラバーサルを防止する
- **Priority**: high

## 解析ノート

GraphQLリゾルバにおける直接文字列結合、未検証シェル呼び出し、認可欠如を重点的に洗い出し。SQLI、RCE、SSRF、IDOR、AFOの5種を特定し、各Principal-Action-Resource間の防御不足をPARモデルで整理。


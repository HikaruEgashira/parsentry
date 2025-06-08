# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `AFO`

## PAR Policy Analysis

### Principals (データ源)

- **process.env.GITHUB_WORKSPACE**: SemiTrusted
  - Context: 環境変数
  - Risk Factors: 外部制御可能な環境変数, ファイル探索の基点として利用

### Actions (セキュリティ制御)

- **findFile(...)**: Insufficient
  - Function: ファイルシステム探索
  - Weaknesses: base パスの検証・正規化が無い, 再帰の深さ制限やサニタイズが無い
  - Bypass Vectors: 環境変数 GITHUB_WORKSPACE を操作して任意ディレクトリを指定可能
- **fs.writeFileSync(...)**: Missing
  - Function: ファイル書き込み／上書き
  - Weaknesses: 認可チェックやファイル整合性検証が無い, 既存コンテンツを破棄し上書きする
  - Bypass Vectors: backdoor-demo フォルダ配下の index.js を検出して無条件に上書き

### Resources (操作対象)

- **<GITHUB_WORKSPACE>/.../backdoor-demo/index.js**: High
  - Operation: file_write
  - Protection: 

### Policy Violations

#### FILE_WRITE_NO_AUTHZ: ファイル書き込みは認可・パス検証を要する

- **Path**: fs.writeFileSync
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

このコードは環境変数 GITHUB_WORKSPACE を基点にファイルシステムを再帰的に探索し、パスに “backdoor-demo” を含むディレクトリ内の index.js を検出したら、その内容を無条件に上書きしています。環境変数やファイル名による制限しかなく、認可チェックやパス正規化、ホワイトリスト検証が欠如しているため、任意のディレクトリを指定して重要コードを改竄される危険があります。

## PoC（概念実証コード）

```text
// POC: 任意のパスを書き込ませる例
process.env.GITHUB_WORKSPACE = "/"; // ルート以下を探索可能にする
// /backdoor-demo/index.js が存在すれば上書きされる
require('./compile.js');
// 実行後、/backdoor-demo/index.js が「// This is a preinstall modification」の1行に置き換わる
```

## 修復ガイダンス

### compile.js

- **Required**: 対象ファイルパスのホワイトリスト検証と正規化、認可チェックを実装
- **Guidance**: 1. GITHUB_WORKSPACE の値を受け入れる前に正規化 (path.resolve) し、許可されたサブディレクトリ内のみを探索対象とする
2. ファイル検出時に改竄許可を持つユーザ／プロセスか認証・認可を行う
3. 既存コンテンツを保持したい場合はプリペンドではなく、必要に応じてバッキングアップ／差分マージ方式を採用
- **Priority**: high

## 解析ノート

- 環境変数を基点に再帰探索→セキュリティホール
- パス検証・認可なしで上書き→任意ファイル改竄可能
- AFO (Arbitrary File Overwrite) 脆弱性
- POC: GITHUB_WORKSPACE を '/' に設定しルート配下の backdoor-demo/index.js を上書き
- 対策: path.resolve, ホワイトリスト, 認可チェックを追加


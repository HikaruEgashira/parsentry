# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **Grunt task argument**: Untrusted
  - Context: Command line argument passed to grunt db-reset task
  - Risk Factors: ["External user input", "Shell metacharacters allowed"]

- **NODE_ENV environment variable**: Semi_trusted  
  - Context: Environment variable used as fallback
  - Risk Factors: ["Process environment", "Configurable by deployment"]

### Actions (セキュリティ制御)

- **child_process.exec**: Missing
  - Function: Execute shell commands for database reset
  - Weaknesses: ["No input validation", "Direct shell execution", "Command injection"]
  - Bypass Vectors: ["Semicolon injection", "Command chaining", "Shell metacharacters"]

### Resources (操作対象)

- **System shell**: Critical
  - Operation: Command execution
  - Protection: []

### Policy Violations

#### RCE-GRUNT-01: 信頼できない入力をシェルコマンドに直接渡している

- **Path**: grunt task argument → finalEnv → child_process.exec
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

Gruntfile.js の "db-reset" タスクで、ユーザーが提供する引数または NODE_ENV 環境変数を検証なしで直接シェルコマンドに連結しています。`child_process.exec()` はシェル経由でコマンドを実行するため、攻撃者がセミコロンやその他のシェルメタ文字を挿入することで任意のコマンドを実行できます。

## PoC（概念実証コード）

```bash
# 攻撃例: Gruntタスクに悪意ある引数を渡す
grunt db-reset="production; touch /tmp/pwned.txt; echo 'hacked' > /tmp/proof.txt; #"

# この結果、以下のコマンドが実行される：
# node artifacts/db-reset.js production; touch /tmp/pwned.txt; echo 'hacked' > /tmp/proof.txt; #
```

## 修復ガイダンス

### child_process.exec の安全化
Required: 入力検証とコマンド実行方式の変更
Guidance: child_process.execFile() を使用し、引数を配列で渡すか、入力値をホワイトリストで検証してシェルメタ文字を除去する
Priority: high

## 解析ノート

1. grunt.registerTask("db-reset") でユーザー引数を直接 finalEnv として使用
2. child_process.exec() でシェル経由実行のため、シェルインジェクション可能
3. 対策: execFile() の使用、または厳格な入力バリデーション（英数字のみ許可）
4. 攻撃シナリオ: `grunt db-reset="test; rm -rf /; #"` など
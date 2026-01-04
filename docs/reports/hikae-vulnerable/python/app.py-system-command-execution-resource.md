# Security Analysis: app.py - System command execution resource

## ファイル情報

- **ファイルパス**: `repo/app.py`
- **検出パターン**: System command execution resource

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 95**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

## マッチしたソースコード

```code
os.system
```

## 詳細解析

## コマンドインジェクション脆弱性

### 脆弱性の詳細

**パターン**: `os.system()` による命令実行

**PAR分析結果**:

#### Principal（実行主体）
- ユーザーからのHTTPリクエストパラメータ `hostname` と `count`
- 320-321行目で `request.args.get()` により取得
- 信頼できないデータソース

#### Resource（リソース）
- 330行目: `os.system(f"ping -c 1 {hostname} > {temp_file}")`
- 326行目: `os.popen(f"ping -c {count} {hostname}")`
- システムシェルによるコマンド実行

#### Action（セキュリティ制御）
- **制御がない**: ユーザー入力に対する検証やサニタイズが完全に欠落
- **直接埋め込み**: ユーザー入力をそのままシェルコマンドに埋め込み

### 攻撃シナリオ

```
GET /cmdi?hostname=localhost;id&count=1
GET /cmdi?hostname=localhost|whoami&count=1
GET /cmdi?hostname=$(cat /etc/passwd)&count=1
GET /cmdi?hostname=`rm -rf /`&count=1
```

### 脆弱性の深刻度

- **CWE**: CWE-78 (Improper Neutralization of Special Elements used in an OS Command)
- **CVSS**: 9.8 (Critical)
- **信頼度**: 95%

ユーザー入力が直接OSコマンドに使用されており、完全なリモートコード実行が可能です。


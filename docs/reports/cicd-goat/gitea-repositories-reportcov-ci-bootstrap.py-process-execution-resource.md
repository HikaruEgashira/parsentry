# Security Analysis: bootstrap.py - Process execution resource

## ファイル情報

- **ファイルパス**: `repo/gitea/repositories/reportcov/ci/bootstrap.py`
- **検出パターン**: Process execution resource

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 75**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **System environment, PATH variable** - Untrusted

### Actions (セキュリティ制御)

- **No validation of tox command location** - Missing

### Resources (操作対象)

- **subprocess.check_output process execution** - Critical

### Policy Violations

- **RCE**: System environment → subprocess.check_output → output processing → template variables → file generation
  - Severity: warning | Confidence: 75%

## マッチしたソースコード

```python
subprocess.check_output
```

## 詳細解析

### RCE脆弱性: subprocess.check_outputによるコマンド実行

**Principal（主体）**: システム環境、PATH環境変数

**Resource（リソース）**: プロセス実行（subprocess.check_output）

**Action（アクション）**: 検証なし

**詳細分析**:

1. **脆弱性箇所**: 63行目
```python
for line in subprocess.check_output(['tox', '--listenvs'], universal_newlines=True).splitlines()
```

2. **攻撃シナリオ**:
   - `tox`がインストールされていないまたはPATHに存在しない場合
   - 攻撃者が同じ名前の悪意あるスクリプトをPATH上に配置
   - スクリプト実行時に悪意あるコードが実行される

3. **Data Flow**:
   - システム環境（Principal）→ subprocess実行 → tox出力取得
   - 取得した環境名を処理（64-71行目）
   - テンプレート変数に格納（68-71行目）
   - ファイル生成時に使用（76行目）

4. **影響範囲**: CI/CDパイプラインの完全な侵害

**推奨対策**:
- 絶対パスでコマンドを指定
- tox実行前に存在確認
- 出力結果のサニタイズ


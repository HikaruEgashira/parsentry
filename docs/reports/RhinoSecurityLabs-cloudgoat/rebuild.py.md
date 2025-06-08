# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `AFO`

## PAR Policy Analysis

### Principals (データ源)

- **filename**: Untrusted
  - Context: rebuild関数の引数
  - Risk Factors: 攻撃者が制御可能, アーカイブ内メンバー操作の起点

### Actions (セキュリティ制御)

- **TarFile.extract**: Insufficient
  - Function: アーカイブ展開
  - Weaknesses: パス検証なしでTarFile.extractを使用
  - Bypass Vectors: ../ (ディレクトリトラバーサル)
- **subprocess.check_call**: Adequate
  - Function: 外部コマンド起動
  - Weaknesses: 
  - Bypass Vectors: 

### Resources (操作対象)

- **tmpdir配下のファイルシステム**: Medium
  - Operation: ファイル書き込み／上書き
  - Protection: 
- **モジュール内ZONEFILENAME**: Medium
  - Operation: ファイル書き込み／上書き
  - Protection: 

### Policy Violations

#### PYSEC100: Tar展開時にパス検証を行わずパス・トラバーサルを許可

- **Path**: rebuild -> TarFile.extract
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

このコードでは、外部から渡されたtarファイル（filename）を検証なしにTarFile.extractで展開しているため、tar内部のメンバー名に"../"などを含めることで任意のファイル上書き（パス・トラバーサル）が可能となっています。tmpdir配下に展開後、最終的にdateutilモジュールのゾーン情報tar（ZONEFILENAME）を書き換える挙動もあるため、攻撃者はモジュールのコードに影響を及ぼすpayloadを注入できます。

## PoC（概念実証コード）

```text
# 実証コード: 悪意あるtar作成サンプル
import tarfile
t = tarfile.open('malicious.tar','w')
# tmpdirの外にファイルを上書き
info = tarfile.TarInfo('../../evil.txt')
info.size = len(b'evil')
t.addfile(info, io.BytesIO(b'evil'))
t.close()
# 生成したmalicious.tarをrebuildに渡すと任意のパスが上書きされる
```

## 修復ガイダンス

### rebuild関数

- **Required**: 安全なtar展開ロジックの導入
- **Guidance**: tarメンバー名に対して絶対パスや'..'を含むものを除去し、安全なパスに正規化したうえで展開する。Python標準のextractallではfilter機能を使うか、メンバーごとに手動検証を行う。
- **Priority**: high

## 解析ノート

1. rebuild()のfilename引数は外部入力かつuntrustedと判断
2. TarFile.extractを検証なしに使用→ディレクトリトラバーサルの可能性
3. tmpdir外へのファイル上書きで任意ファイル操作リスク
4. _run_zicのcheck_callはリスト渡しで安全
5. ポリシー違反としてTar展開前のパスバリデーション不足を報告
6. 攻撃例として../を含むメンバーでevil.txt上書きを記載


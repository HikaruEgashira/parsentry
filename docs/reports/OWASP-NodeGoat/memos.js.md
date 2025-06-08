# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **req.body.memo**: Untrusted
  - Context: HTTP リクエストボディ
  - Risk Factors: 改ざん可能, スクリプト挿入
- **req.session.userId**: SemiTrusted
  - Context: セッションストア
  - Risk Factors: セッション乗っ取りリスク

### Actions (セキュリティ制御)

- **memosDAO.insert**: Missing
  - Function: Input Validation
  - Weaknesses: 入力検証欠如, サニタイズ欠如
  - Bypass Vectors: 
- **res.render**: Missing
  - Function: Output Encoding
  - Weaknesses: 出力エンコーディング欠如
  - Bypass Vectors: 

### Resources (操作対象)

- **memos collection**: Medium
  - Operation: database_insert
  - Protection: 
- **layout テンプレート**: Medium
  - Operation: template_render
  - Protection: 

### Policy Violations

#### XSS-001: ユーザー入力をエスケープせずに HTML 出力に含めている（Stored XSS）

- **Path**: MemosHandler.addMemos -> memosDAO.insert(req.body.memo) -> displayMemos -> res.render()
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

このコードでは、ユーザーからの untrusted な入力（req.body.memo）に対してバリデーションやサニタイズを行わずにデータベースへそのまま保存し、後続のテンプレートレンダリング時にもエスケープ処理がないため、ストアド XSS が発生する可能性があります。

## PoC（概念実証コード）

```text
// 攻撃者が次のようなリクエストを送信
POST /memos
Content-Type: application/json

{"memo":"<script>alert('XSS');</script>"}

// 保存後、/memos ページを表示するとアラートが発動
```

## 修復ガイダンス

### Input Validation

- **Required**: ユーザー入力の検証・ホワイトリストサニタイズを実装
- **Guidance**: memo の最大長や許可文字を制限し、想定外文字を拒否する
- **Priority**: high

### Output Encoding

- **Required**: テンプレートでのエスケープ（HTML エンコーディング）を適用
- **Guidance**: 使用中のテンプレートエンジンの自動エスケープ機能を有効化するか、明示的にエスケープ関数を呼び出す
- **Priority**: high

## 解析ノート

1. req.body.memo を Principal（untrusted）として特定
2. memosDAO.insert には入力検証が実装されていない（Action.implementation_quality=missing）
3. res.render にもエスケープ処理がない（Action.implementation_quality=missing）
4. 結果として Stored XSS 脆弱性を検出
5. remediation で入力検証と出力エンコーディングを推奨


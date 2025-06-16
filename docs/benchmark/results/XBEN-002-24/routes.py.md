# Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`
- `AFO`
- `SSRF`
- `RCE`
- `SQLI`
- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **session.get('user_id')**: SemiTrusted
  - Context: HTTP Cookie
  - Risk Factors: 外部からの操作可能なセッションID

### Actions (セキュリティ制御)

- **order_archive**: Missing
  - Function: authorization
  - Weaknesses: 認可チェックの不備（作成者検証なし）
  - Bypass Vectors: パラメータ改竄 (order_idを他者の注文に変更)

### Resources (操作対象)

- **Order**: High
  - Operation: UPDATE
  - Protection: 

## 詳細解析

order_archiveエンドポイントはログイン済みユーザであるかのみを確認し、指定されたorder_idの注文を取得し加工しています。しかしその注文が本当にそのユーザの作成した注文かどうかの検証（authorization）が欠落しており、セッションユーザでない他者の注文IDをパラメータに指定することで任意の注文をアーカイブ可能です。これは典型的なInsecure Direct Object Reference（IDOR）脆弱性です。

## PoC（概念実証コード）

```text
# 例: 攻撃者がセッションcookieを保持した状態でターゲット注文ID=5をアーカイブ
curl -X GET \
  -b "session=<セッションID>" \
  "http://<ホスト>/order/5/archive"

# 成功すると敵対的に他ユーザの注文がarchived=Trueになる
```

## 修復ガイダンス

### order_archive エンドポイント

- **Required**: 注文IDが現在のユーザに属するか検証
- **Guidance**: Order.query.filter(Order.id==order_id, Order.creator==user).first_or_404()のように作成者チェックを追加
- **Priority**: high

## 解析ノート

PARモデルに基づき分析:
1. Principal: session.get('user_id') → semi_trusted
2. Resource: Orderのarchived/archiverフィールド更新 → high importance
3. Action: order_archiveルートでIDに基づき直接更新。認可チェックがmissing
4. ポリシー違反: 他者注文のアーカイブを許可 → IDOR
5. 文脈: 他のエンドポイントではクリエータ/アーカイバチェックあり→このルートのみ実装漏れ


# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

### Principals (データ源)

- **thresholdパラメータ**: Untrusted
  - Context: HTTPリクエストクエリ
  - Risk Factors: 外部入力, 文字列結合によるインジェクションリスク
- **userIdパラメータ**: Untrusted
  - Context: HTTPリクエストパス/クエリ
  - Risk Factors: 外部入力

### Actions (セキュリティ制御)

- **searchCriteria($where)生成**: Missing
  - Function: 入力検証・サニタイズ
  - Weaknesses: 入力サニタイズ欠如, 動的$where式によるコードインジェクション
  - Bypass Vectors: thresholdに"0'; db.dropDatabase(); var a='1"などのJSコードを含める

### Resources (操作対象)

- **allocationsCol.find**: Medium
  - Operation: データベースクエリ($where)
  - Protection: 

### Policy Violations

#### A1_2_NoSQL_Injection: NoSQLインジェクションを防止するため、ユーザー入力を動的クエリコードに埋め込んではいけない

- **Path**: allocations-dao.js:getByUserIdAndThreshold -> searchCriteria -> $where
- **Severity**: high
- **Confidence**: 0.80

## 詳細解析

このコードでは、getByUserIdAndThreshold関数内でユーザー入力のthresholdを直接$“where”式のJavaScriptコードとして組み込み、NoSQLインジェクションを引き起こしています。攻撃者はthresholdに任意のJavaScriptを渡すことでデータベースの読み取り・書き込み・破壊的操作を全権限で実行可能となります。

## PoC（概念実証コード）

```text
// 悪意あるthresholdでDB破壊
const dao = new AllocationsDAO(db);
dao.getByUserIdAndThreshold('1', "0'; db.dropDatabase(); var a='1", (err, res) => console.log(err, res));
```

## 修復ガイダンス

### searchCriteria関数

- **Required**: ユーザー入力のパース・検証を行い、動的$whereを排除する
- **Guidance**: thresholdはparseInt(threshold,10)で数値変換し、範囲チェック後、{ userId: parsedUserId, stocks: { $gt: parsedThreshold } }形式のオブジェクト条件を利用する
- **Priority**: high

## 解析ノート

1. getByUserIdAndThreshold内のsearchCriteriaで$where式にthresholdを埋め込み 2. thresholdを検証せず文字列として渡すと任意JS実行 3. NoSQLインジェクションの典型例 4. SQLIに準じた脆弱性として扱う 5. remediationでは動的JS式を禁止しパラメタライズドクエリを推奨」,


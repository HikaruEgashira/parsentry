# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **ユーザー入力**: Untrusted
  - Context: $.html(), $.append() などで受け取る文字列
  - Risk Factors: 外部から供給されるHTML文字列, 信頼されないコンテキスト

### Actions (セキュリティ制御)

- **jQuery.fn.html / DOMEval**: Insufficient
  - Function: 不特定入力のサニタイズ・エスケープ
  - Weaknesses: 未検証HTMLをinnerHTMLで直接挿入, globalEvalによる動的コード実行
  - Bypass Vectors: 悪意ある<script>タグ挿入, イベントハンドラ属性埋め込み

### Resources (操作対象)

- **ブラウザDOM**: High
  - Operation: innerHTMLによるDOM挿入／script要素評価
  - Protection: 

### Policy Violations

#### XSS001: ユーザー入力をサニタイズせずinnerHTML/globalEvalに渡すことでスクリプトが実行される

- **Path**: jQuery.fn.html -> buildFragment -> innerHTML -> DOMEval
- **Severity**: high
- **Confidence**: 0.80

## 詳細解析

このjQueryライブラリでは、ユーザー供給データを検証・サニタイズせずにDOM操作やグローバルスクリプト評価を行う関数（html(), append(), globalEval()等）が存在します。これにより、悪意あるHTML/JavaScriptを挿入することでXSSが発生するリスクがあります。

## PoC（概念実証コード）

```text
// 悪意ある入力例
document.body.innerHTML = '<div id="target"></div>';
$('#target').html('<img src=x onerror="alert(1)">');
```

## 修復ガイダンス

### html関連メソッド(html(), append(), htmlPrefilter)

- **Required**: ユーザー入力HTMLのサニタイズ
- **Guidance**: DOMPurify等のライブラリで不正なタグやイベント属性を除去してからhtml()に渡す
- **Priority**: high

### グローバルEval(DOMEval, globalEval)

- **Required**: 動的コード評価を廃止または安全化
- **Guidance**: ユーザーデータをevalに渡さない。必要ならテンプレートエンジンや安全サンドボックスを利用
- **Priority**: high

## 解析ノート

jQueryでは多数のDOM操作API(html, append, wrapなど)がinnerHTMLやcreateElement/appendChild経由で文字列を直接挿入し、buildFragment経由で<script>タグを生成しDOMEvalで評価する。これをユーザー供給データで呼び出すとXSSとなる。ライブラリ側でサニタイズを行っておらず、glovalEvalが未検証文字列を評価する実装不備がある。従って未検証HTMLを扱うアプリケーションコードには必ずホワイトリストベースのサニタイズが必要。RORCEは該当せず、主脆弱性はXSS。ポリシー違反は"ユーザー入力を直接innerHTML/globalEvalに渡す"。 例:$(...).html(userInput) -> XSS.


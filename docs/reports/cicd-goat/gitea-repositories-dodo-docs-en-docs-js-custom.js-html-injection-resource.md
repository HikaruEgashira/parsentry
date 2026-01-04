# Security Analysis: custom.js - HTML injection resource

## ファイル情報

- **ファイルパス**: `repo/gitea/repositories/dodo/docs/en/docs/js/custom.js`
- **検出パターン**: HTML injection resource

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **External GitHub API response (v.html_url, v.full_name, v.owner.html_url, v.owner.login)** - Untrusted

### Actions (セキュリティ制御)

- **No input validation, sanitization, or output escaping applied** - Missing

### Resources (操作対象)

- **Browser DOM via innerHTML assignment (line 170)** - Critical

### Policy Violations

- **XSS**: GitHub API → getDataBatch() → getData() → main() → data.forEach() → li.innerHTML with unsanitized external data
  - Severity: error | Confidence: 90%

## マッチしたソースコード

```javascript
innerHTML
```

## 詳細解析

## Cross-Site Scripting (XSS) 脆弱性

### 問題
GitHub APIから取得した外部データが、入力検証やサニタイズなしに直接 DOM の `innerHTML` プロパティに挿入されています。

### 脆弱性の詳細

**Principal（信頼できないデータソース）:**
- `https://api.github.com/search/repositories?q=topic:fastapi&per_page=100&page=${page}` からのレスポンス
- 特に以下のフィールドが危険:
  - `v.html_url`: リポジトリの URL
  - `v.full_name`: リポジトリのフルネーム
  - `v.owner.html_url`: オーナーの URL
  - `v.owner.login`: オーナーのログイン名

**Resource（保護されるべきリソース）:**
- ブラウザの DOM
- ページのスクリプト実行コンテキスト

**Action（セキュリティ制御）:**
- ✗ 入力検証なし
- ✗ 出力エスケープなし
- ✗ サニタイズなし
- ✗ Content Security Policy (CSP) による保護なし

### 攻撃シナリオ

1. 攻撃者が GitHub でマルウェア リポジトリを作成
2. リポジトリ名に XSS ペイロードを含める: `<img src=x onerror="alert('XSS')">` または `<script>...</script>`
3. ユーザーがこのドキュメントページを訪問
4. `main()` 関数が GitHub API からリポジトリデータを取得
5. リポジトリ名（XSS ペイロード）が `innerHTML` で HTML として解析・実行される
6. ページの閲覧者のセッション、クッキー、または認証情報が窃取される

### 技術的詳細

**脆弱なコード (行170):**
```javascript
li.innerHTML = \`<a href="${v.html_url}" target="_blank">★ ${v.stargazers_count} - ${v.full_name}</a> by <a href="${v.owner.html_url}" target="_blank">@${v.owner.login}</a>\`
```

`v.full_name` が `fastapi/<img src=x onerror="console.log('xss')">/repo` のような値の場合、HTML として実行されます。

同様に行163も危険:
```javascript
div.innerHTML = '<ul></ul>'
```

### 攻撃可能性
- **HTTP Request: T1190** - 外部エンティティを利用した攻撃
- **Exploitation for Client Execution: T1203** (MITRE ATT&CK) - クライアントブラウザでの任意コード実行

### 推奨される修正

1. **`innerHTML` を `textContent` に変更:**
   ```javascript
   li.textContent = \`...\`  // テキストのみを設定
   ```

2. **DOM API を使用して安全に要素を構築:**
   ```javascript
   const a1 = document.createElement('a');
   a1.href = v.html_url;
   a1.target = '_blank';
   a1.textContent = \`★ ${v.stargazers_count} - ${v.full_name}\`;
   
   const a2 = document.createElement('a');
   a2.href = v.owner.html_url;
   a2.target = '_blank';
   a2.textContent = \`@${v.owner.login}\`;
   
   li.appendChild(a1);
   li.appendChild(document.createTextNode(' by '));
   li.appendChild(a2);
   ```

3. **DOMPurify などのライブラリを使用:**
   ```javascript
   li.innerHTML = DOMPurify.sanitize(htmlString);
   ```

### CVSS スコア
- **ベクトル: CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:C/C:H/I:H/A:N**
- **スコア: 8.7 (高)**

### CWE
- CWE-79: Improper Neutralization of Input During Web Page Generation ('Cross-site Scripting')

### OWASP Top 10
- **A03:2021 - Injection**
- **A07:2021 - Cross-Site Scripting (XSS)**



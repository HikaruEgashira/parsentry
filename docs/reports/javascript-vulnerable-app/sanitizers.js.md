# 解析レポート

![中信頼度](https://img.shields.io/badge/信頼度-中-yellow) **信頼度スコア: 60**

## 脆弱性タイプ

- `XSS`
- `SSRF`
- `AFO`
- `SQLI`

## 解析結果

本実装ではサニタイズ／バリデーション関数が用意されていますが、いずれも不完全であり以下の脆弱性を残しています。

1. XSS（クロスサイトスクリプティング）
   - sanitizeContentでは<script>タグとon属性のみを置換／除去しているが、javascript:スキーマを含むhref属性やiframe、SVGタグなどは無視されるため、依然として悪意あるスクリプト実行が可能。

2. SSRF（サーバーサイドリクエストフォージェリ）
   - validateExternalURLではホスト名や特定のプライベートIPパターンを正規表現でチェックしているが、16進数、10進数などのIPエンコーディングやリダイレクトサービスを排除せず、SSRFリクエストが可能。

3. AFO（任意ファイルアップロード）
   - validateUploadではファイル名の末尾拡張子だけをチェックし、多重拡張子（shell.php.jpg）やContent-Type偽装、拡張子付与後のコード混入を正しく検出できないため、サーバー側での任意コード実行リスク。

4. SQLI（SQLインジェクション）
   - validateSQLではSQLキーワードを単純に"***"へ置換しコメント文字を除去しているが、プレースホルダやパラメータ化を行わず、エスケープ漏れやキーワードの回避手法（バイパスコメント、別スキーマ）に弱い。

各サニタイズ／バリデーション関数は安全性を担保するものではなく、実際には適切なエスケープ・パラメータ化・ホワイトリスト検証を組み合わせる必要があります。

## PoC（概念実証コード）

```text
1) XSS: <a href="javascript:alert(1)">click</a>
2) SSRF: http://0x7f000001 (127.0.0.1の16進表記)
3) AFO: ファイル名「shell.php.jpg」にPHPコードを埋め込みアップロード
4) SQLI: バイパスコメントを用いた「uni/**/on select * from users」
```

## 関連コードコンテキスト

### 関数名: sanitizeContent
- 理由: <script>タグのみ除去し、javascript:スキーマやiframe、SVGなどは無視しているためXSSバイパス可能
- パス: repo/utils/sanitizers.js
```rust
cleaned = cleaned.replace(/<script[^>]*>/gi, '');
```

### 関数名: validateExternalURL
- 理由: IPアドレスの16進／10進エンコーディングやリダイレクト先が検出対象外であり、SSRFが可能
- パス: repo/utils/sanitizers.js
```rust
const restrictedPatterns = [ /localhost/i, /127\.0\.0\.1/, /192\.168\./, /10\./, /172\.(1[6-9]|2[0-9]|3[0-1])\./ ];
```

### 関数名: validateUpload
- 理由: 末尾拡張子だけチェックしており、多重拡張子やContent-Type偽装を回避できずAFOを許容
- パス: repo/utils/sanitizers.js
```rust
const ext = filename.toLowerCase().split('.').pop();
```

### 関数名: validateSQL
- 理由: キーワード単純置換のみでパラメータ化せず、SQLインジェクションを根本的に防げない
- パス: repo/utils/sanitizers.js
```rust
filtered = filtered.replace(new RegExp(`\\b${keyword}\\b`, 'gi'), '***');
```

## 解析ノート

各関数の不完全な検証を列挙し、具体的な脆弱性と回避手法（バイパス）を確認


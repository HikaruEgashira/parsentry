# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [gitea.py (SQL execution resource)](gitea-giteacasc-gitea.py-sql-execution-resource.md) | SQLI | 🔴 高 | SQLI |
| [gitea.py (SQL execution resource)](gitea-giteacasc-gitea.py-sql-execution-resource.md) | SQLI | 🔴 高 | SQLI |
| [gitea.py (SQL execution resource)](gitea-giteacasc-gitea.py-sql-execution-resource.md) | SQLI | 🔴 高 | SQLI |
| [custom.js (HTML injection resource)](gitea-repositories-dodo-docs-en-docs-js-custom.js-html-injection-resource.md) | XSS | 🔴 高 | XSS |
| [custom.js (DOM access operations)](gitea-repositories-dodo-docs-en-docs-js-custom.js-dom-access-operations.md) | XSS | 🔴 高 | XSS |
| [custom.js (DOM access operations)](gitea-repositories-dodo-docs-en-docs-js-custom.js-dom-access-operations.md) | XSS | 🔴 高 | XSS |
| [custom.js (HTML injection resource)](gitea-repositories-dodo-docs-en-docs-js-custom.js-html-injection-resource.md) | XSS | 🔴 高 | XSS |
| [oauth2.py (File operations resource)](gitea-repositories-mad-hatter-yagmail-oauth2.py-file-operations-resource.md) | AFO | 🔴 高 | AFO |
| [custom.js (DOM access operations)](gitea-repositories-dodo-docs-en-docs-js-custom.js-dom-access-operations.md) | XSS | 🔴 高 | XSS |
| [export.py (File operations resource)](ctfd-data-CTFd-export.py-file-operations-resource.md) | AFO | 🔴 高 | AFO |
| [custom.js (DOM access operations)](gitea-repositories-dodo-docs-en-docs-js-custom.js-dom-access-operations.md) | XSS | 🟠 中高 | XSS |
| [custom.js (DOM access operations)](gitea-repositories-dodo-docs-en-docs-js-custom.js-dom-access-operations.md) | XSS | 🟠 中高 | XSS |
| [oauth2.py (File operations resource)](gitea-repositories-mad-hatter-yagmail-oauth2.py-file-operations-resource.md) | AFO | 🟠 中高 | AFO |
| [main.tf (AWS IAM resources)](gitea-repositories-dodo-main.tf-aws-iam-resources.md) | Other("OVERPERM") | 🟠 中高 | OVERPERM |
| [main.tf (AWS S3 bucket resources)](gitea-repositories-dodo-main.tf-aws-s3-bucket-resources.md) | Other("AWS-S3-ENCRYPTION") | 🟠 中高 | AWS-S3-ENCRYPTION |
| [main.tf (AWS IAM resources)](gitea-repositories-dodo-main.tf-aws-iam-resources.md) | Other("IAM_PRIVILEGE_ESCALATION") | 🟠 中高 | IAM_PRIVILEGE_ESCALATION |
| [__init__.py (Code execution resource)](gitea-repositories-mock-turtle-eel-__init__.py-code-execution-resource.md) | RCE | 🟠 中高 | RCE |
| [reloader_helpers.py (Process execution resource)](gitea-repositories-cheshire-cat-sanic-reloader_helpers.py-process-execution-resource.md) | RCE | 🟠 中高 | RCE |
| [__init__.py (Code execution resource)](gitea-repositories-mock-turtle-eel-__init__.py-code-execution-resource.md) | RCE | 🟠 中高 | RCE |
| [__init__.py (SQL execution resource)](ctfd-data-CTFd-utils-exports-__init__.py-sql-execution-resource.md) | SQLI | 🟠 中高 | SQLI |
| [__init__.py (SQL execution resource)](ctfd-data-CTFd-utils-exports-__init__.py-sql-execution-resource.md) | SQLI | 🟠 中高 | SQLI |
| [setup.js (JSON parsing operations)](ctfd-data-CTFd-themes-core-assets-js-pages-setup.js-json-parsing-operations.md) | Other("PARSE_ERROR") | 🟠 中高 | PARSE_ERROR |
| [__init__.py (SQL execution resource)](ctfd-data-CTFd-utils-exports-__init__.py-sql-execution-resource.md) | SQLI | 🟠 中高 | SQLI |
| [__init__.py (SQL execution resource)](ctfd-data-CTFd-CTFd-__init__.py-sql-execution-resource.md) | SQLI | 🟠 中高 | SQLI |
| [__init__.py (SQL execution resource)](ctfd-data-CTFd-CTFd-utils-__init__.py-sql-execution-resource.md) | SQLI | 🟠 中高 | SQLI |
| [__init__.py (SQL execution resource)](ctfd-data-CTFd-CTFd-utils-exports-__init__.py-sql-execution-resource.md) | SQLI | 🟠 中高 | SQLI |
| [__init__.py (SQL execution resource)](ctfd-data-CTFd-CTFd-utils-exports-__init__.py-sql-execution-resource.md) | SQLI | 🟠 中高 | SQLI |
| [__init__.py (SQL execution resource)](ctfd-data-CTFd-CTFd-utils-exports-__init__.py-sql-execution-resource.md) | SQLI | 🟠 中高 | SQLI |
| [setup.js (JSON parsing operations)](ctfd-data-CTFd-CTFd-themes-core-assets-js-pages-setup.js-json-parsing-operations.md) | Other("PARSE_ERROR") | 🟠 中高 | PARSE_ERROR |
| [configs.js (DOM access operations)](ctfd-data-CTFd-themes-admin-assets-js-pages-configs.js-dom-access-operations.md) | XSS | 🟠 中高 | XSS |
| [challenge.js (DOM access operations)](ctfd-data-CTFd-themes-admin-assets-js-pages-challenge.js-dom-access-operations.md) | XSS | 🟠 中高 | XSS |
| [main.tf (AWS S3 bucket resources)](gitea-repositories-dodo-main.tf-aws-s3-bucket-resources.md) | AFO | 🟠 中高 | AFO |
| [oauth2.py (File operations resource)](gitea-repositories-mad-hatter-yagmail-oauth2.py-file-operations-resource.md) | AFO | 🟠 中高 | AFO |
| [bootstrap.py (Process execution resource)](gitea-repositories-reportcov-ci-bootstrap.py-process-execution-resource.md) | RCE | 🟠 中高 | RCE |
| [configs.js (DOM access operations)](ctfd-data-CTFd-themes-admin-assets-js-pages-configs.js-dom-access-operations.md) | XSS | 🟠 中高 | XSS |
| [configs.js (DOM access operations)](ctfd-data-CTFd-themes-admin-assets-js-pages-configs.js-dom-access-operations.md) | XSS | 🟠 中高 | XSS |
| [configs.js (DOM access operations)](ctfd-data-CTFd-CTFd-themes-admin-assets-js-pages-configs.js-dom-access-operations.md) | XSS | 🟠 中高 | XSS |
| [configs.js (DOM access operations)](ctfd-data-CTFd-CTFd-themes-admin-assets-js-pages-configs.js-dom-access-operations.md) | XSS | 🟠 中高 | XSS |
| [configs.js (DOM access operations)](ctfd-data-CTFd-CTFd-themes-admin-assets-js-pages-configs.js-dom-access-operations.md) | XSS | 🟠 中高 | XSS |
| [configs.js (DOM access operations)](ctfd-data-CTFd-CTFd-themes-admin-assets-js-pages-configs.js-dom-access-operations.md) | XSS | 🟠 中高 | XSS |
| [configs.js (DOM access operations)](ctfd-data-CTFd-CTFd-themes-admin-assets-js-pages-configs.js-dom-access-operations.md) | XSS | 🟠 中高 | XSS |
| [configs.js (DOM access operations)](ctfd-data-CTFd-CTFd-themes-admin-assets-js-pages-configs.js-dom-access-operations.md) | XSS | 🟠 中高 | XSS |
| [securetransport.py (File operations resource)](gitea-repositories-white-rabbit-src-urllib3-contrib-securetransport.py-file-operations-resource.md) | AFO | 🟠 中高 | AFO |
| [ssl_.py (File operations resource)](gitea-repositories-white-rabbit-src-urllib3-util-ssl_.py-file-operations-resource.md) | AFO | 🟠 中高 | AFO |
| [securetransport.py (File operations resource)](gitea-repositories-white-rabbit-src-urllib3-contrib-securetransport.py-file-operations-resource.md) | AFO | 🟠 中高 | AFO |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| PARSE_ERROR | 2 | Unsafe data flow: localStorage → storage event → JSON.parse() → DOM update |
| OVERPERM | 1 | Unsafe data flow: S3 Service → AssumeRole (unconditioned) → IAM Role with excessive S3 permissions → S3 Bucket resources (wildcard) |
| SQLI | 11 | Unsafe data flow: create_token(name, token) -> f-string -> con.execute(INSERT) -> access_token table |
| AWS-S3-ENCRYPTION | 1 | Unsafe data flow: Resource definition → unencrypted S3 bucket creation |
| XSS | 17 | Unsafe data flow: GitHub API → getDataBatch() → getData() → main() → data.forEach() → li.innerHTML with unsanitized external data |
| AFO | 8 | Unsafe data flow: oauth2_file parameter → os.path.expanduser() → open()/write operations |
| RCE | 4 | Unsafe data flow: JS file content → EXPOSED_JS_FUNCTIONS parser → _js_functions list → _import_js_function() → exec() → global scope |
| IAM_PRIVILEGE_ESCALATION | 1 | Unsafe data flow: Infrastructure config → IAM policy → S3 bucket permissions |

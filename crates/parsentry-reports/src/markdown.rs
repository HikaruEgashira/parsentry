use parsentry_core::Response;

pub fn to_markdown(response: &Response) -> String {
    let mut md = String::new();

    // Enhanced title with file and pattern information
    let title = if let (Some(file_path), Some(pattern)) =
        (&response.file_path, &response.pattern_description)
    {
        format!(
            "# Security Analysis: {} - {}",
            file_path.split('/').next_back().unwrap_or(file_path),
            pattern
        )
    } else if let Some(file_path) = &response.file_path {
        format!(
            "# Security Analysis: {}",
            file_path.split('/').next_back().unwrap_or(file_path)
        )
    } else {
        "# Security Analysis Report".to_string()
    };
    md.push_str(&title);
    md.push_str("\n\n");

    // File information section
    if let Some(file_path) = &response.file_path {
        md.push_str("## ファイル情報\n\n");
        md.push_str(&format!("- **ファイルパス**: `{}`\n", file_path));
        if let Some(pattern) = &response.pattern_description {
            md.push_str(&format!("- **検出パターン**: {}\n", pattern));
        }
        md.push('\n');
    }

    let confidence_badge = match response.confidence_score {
        90..=100 => "![高信頼度](https://img.shields.io/badge/信頼度-高-red)",
        70..=89 => "![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange)",
        50..=69 => "![中信頼度](https://img.shields.io/badge/信頼度-中-yellow)",
        30..=49 => "![中低信頼度](https://img.shields.io/badge/信頼度-中低-green)",
        _ => "![低信頼度](https://img.shields.io/badge/信頼度-低-blue)",
    };
    md.push_str(&format!(
        "{} **信頼度スコア: {}**\n\n",
        confidence_badge, response.confidence_score
    ));

    if !response.vulnerability_types.is_empty() {
        md.push_str("## 脆弱性タイプ\n\n");
        for vuln_type in &response.vulnerability_types {
            md.push_str(&format!("- `{:?}`\n", vuln_type));
        }
        md.push('\n');
    }

    // Source code sections
    if let Some(matched_code) = &response.matched_source_code
        && !matched_code.trim().is_empty()
    {
        let lang = response
            .file_path
            .as_ref()
            .and_then(|p| p.split('.').next_back())
            .map(|ext| match ext {
                "rb" => "ruby",
                "py" => "python",
                "js" => "javascript",
                "ts" => "typescript",
                "go" => "go",
                "rs" => "rust",
                "java" => "java",
                "c" | "h" => "c",
                "cpp" | "cc" | "cxx" | "hpp" => "cpp",
                "php" => "php",
                "sh" | "bash" => "bash",
                "tf" => "hcl",
                "yaml" | "yml" => "yaml",
                "json" => "json",
                _ => ext,
            })
            .unwrap_or("text");

        md.push_str("## マッチしたソースコード\n\n");
        md.push_str(&format!("```{}\n", lang));
        md.push_str(matched_code);
        md.push_str("\n```\n\n");
    }

    md.push_str("## 詳細解析\n\n");
    md.push_str(&response.analysis);
    md.push_str("\n\n");

    if !response.poc.is_empty() {
        md.push_str("## PoC\n\n");
        md.push_str("```text\n");
        md.push_str(&response.poc);
        md.push_str("\n```\n\n");
    }

    if !response.scratchpad.is_empty() {
        md.push_str("## 解析ノート\n\n");
        md.push_str(&response.scratchpad);
        md.push_str("\n\n");
    }

    md
}

#[cfg(test)]
mod tests {
    use super::*;
    use parsentry_core::VulnType;

    fn make_empty_response() -> Response {
        Response {
            analysis: "test analysis".to_string(),
            confidence_score: 50,
            ..Default::default()
        }
    }

    fn make_full_response() -> Response {
        Response {
            scratchpad: "thinking notes".to_string(),
            analysis: "detailed analysis".to_string(),
            poc: "curl http://evil".to_string(),
            confidence_score: 95,
            vulnerability_types: vec![VulnType::SQLI, VulnType::XSS],
            file_path: Some("src/app/routes.py".to_string()),
            pattern_description: Some("SQL query construction".to_string()),
            matched_source_code: Some(
                "query = f\"SELECT * FROM users WHERE id={user_id}\"".to_string(),
            ),
            ..Default::default()
        }
    }

    #[test]
    fn test_title_with_file_and_pattern() {
        let r = make_full_response();
        let md = to_markdown(&r);
        assert!(md.starts_with("# Security Analysis: routes.py - SQL query construction\n"));
    }

    #[test]
    fn test_title_with_file_only() {
        let mut r = make_empty_response();
        r.file_path = Some("src/handler.rs".to_string());
        let md = to_markdown(&r);
        assert!(md.starts_with("# Security Analysis: handler.rs\n"));
    }

    #[test]
    fn test_title_no_file() {
        let r = make_empty_response();
        let md = to_markdown(&r);
        assert!(md.starts_with("# Security Analysis Report\n"));
    }

    #[test]
    fn test_confidence_badge_high() {
        let mut r = make_empty_response();
        r.confidence_score = 95;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-高-red"));
    }

    #[test]
    fn test_confidence_badge_90_boundary() {
        let mut r = make_empty_response();
        r.confidence_score = 90;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-高-red"));
    }

    #[test]
    fn test_confidence_badge_100_boundary() {
        let mut r = make_empty_response();
        r.confidence_score = 100;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-高-red"));
    }

    #[test]
    fn test_confidence_badge_medium_high() {
        let mut r = make_empty_response();
        r.confidence_score = 75;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-中高-orange"));
    }

    #[test]
    fn test_confidence_badge_70_boundary() {
        let mut r = make_empty_response();
        r.confidence_score = 70;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-中高-orange"));
    }

    #[test]
    fn test_confidence_badge_89_boundary() {
        let mut r = make_empty_response();
        r.confidence_score = 89;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-中高-orange"));
    }

    #[test]
    fn test_confidence_badge_medium() {
        let mut r = make_empty_response();
        r.confidence_score = 55;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-中-yellow"));
    }

    #[test]
    fn test_confidence_badge_50_boundary() {
        let mut r = make_empty_response();
        r.confidence_score = 50;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-中-yellow"));
    }

    #[test]
    fn test_confidence_badge_69_boundary() {
        let mut r = make_empty_response();
        r.confidence_score = 69;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-中-yellow"));
    }

    #[test]
    fn test_confidence_badge_medium_low() {
        let mut r = make_empty_response();
        r.confidence_score = 40;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-中低-green"));
    }

    #[test]
    fn test_confidence_badge_30_boundary() {
        let mut r = make_empty_response();
        r.confidence_score = 30;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-中低-green"));
    }

    #[test]
    fn test_confidence_badge_49_boundary() {
        let mut r = make_empty_response();
        r.confidence_score = 49;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-中低-green"));
    }

    #[test]
    fn test_confidence_badge_low() {
        let mut r = make_empty_response();
        r.confidence_score = 10;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-低-blue"));
    }

    #[test]
    fn test_confidence_badge_29_is_low() {
        let mut r = make_empty_response();
        r.confidence_score = 29;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度-低-blue"));
    }

    #[test]
    fn test_confidence_score_value_displayed() {
        let mut r = make_empty_response();
        r.confidence_score = 85;
        let md = to_markdown(&r);
        assert!(md.contains("信頼度スコア: 85"));
    }

    #[test]
    fn test_file_info_section_present() {
        let mut r = make_empty_response();
        r.file_path = Some("src/main.py".to_string());
        r.pattern_description = Some("dangerous pattern".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("## ファイル情報"));
        assert!(md.contains("**ファイルパス**: `src/main.py`"));
        assert!(md.contains("**検出パターン**: dangerous pattern"));
    }

    #[test]
    fn test_file_info_section_absent_when_no_file() {
        let r = make_empty_response();
        let md = to_markdown(&r);
        assert!(!md.contains("## ファイル情報"));
    }

    #[test]
    fn test_vuln_types_section() {
        let mut r = make_empty_response();
        r.vulnerability_types = vec![VulnType::SQLI, VulnType::RCE];
        let md = to_markdown(&r);
        assert!(md.contains("## 脆弱性タイプ"));
        assert!(md.contains("- `SQLI`"));
        assert!(md.contains("- `RCE`"));
    }

    #[test]
    fn test_vuln_types_section_absent_when_empty() {
        let r = make_empty_response();
        let md = to_markdown(&r);
        assert!(!md.contains("## 脆弱性タイプ"));
    }

    #[test]
    fn test_lang_ruby() {
        let mut r = make_empty_response();
        r.file_path = Some("app.rb".to_string());
        r.matched_source_code = Some("puts 'hello'".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```ruby\n"));
    }

    #[test]
    fn test_lang_python() {
        let mut r = make_empty_response();
        r.file_path = Some("app.py".to_string());
        r.matched_source_code = Some("print('hello')".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```python\n"));
    }

    #[test]
    fn test_lang_javascript() {
        let mut r = make_empty_response();
        r.file_path = Some("app.js".to_string());
        r.matched_source_code = Some("console.log('hi')".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```javascript\n"));
    }

    #[test]
    fn test_lang_typescript() {
        let mut r = make_empty_response();
        r.file_path = Some("app.ts".to_string());
        r.matched_source_code = Some("const x: number = 1".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```typescript\n"));
    }

    #[test]
    fn test_lang_go() {
        let mut r = make_empty_response();
        r.file_path = Some("main.go".to_string());
        r.matched_source_code = Some("fmt.Println()".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```go\n"));
    }

    #[test]
    fn test_lang_rust() {
        let mut r = make_empty_response();
        r.file_path = Some("main.rs".to_string());
        r.matched_source_code = Some("fn main() {}".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```rust\n"));
    }

    #[test]
    fn test_lang_java() {
        let mut r = make_empty_response();
        r.file_path = Some("App.java".to_string());
        r.matched_source_code = Some("class App {}".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```java\n"));
    }

    #[test]
    fn test_lang_c() {
        let mut r = make_empty_response();
        r.file_path = Some("main.c".to_string());
        r.matched_source_code = Some("int main() {}".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```c\n"));
    }

    #[test]
    fn test_lang_c_header() {
        let mut r = make_empty_response();
        r.file_path = Some("main.h".to_string());
        r.matched_source_code = Some("#include <stdio.h>".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```c\n"));
    }

    #[test]
    fn test_lang_cpp() {
        let mut r = make_empty_response();
        r.file_path = Some("main.cpp".to_string());
        r.matched_source_code = Some("std::cout".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```cpp\n"));
    }

    #[test]
    fn test_lang_cc() {
        let mut r = make_empty_response();
        r.file_path = Some("main.cc".to_string());
        r.matched_source_code = Some("code".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```cpp\n"));
    }

    #[test]
    fn test_lang_cxx() {
        let mut r = make_empty_response();
        r.file_path = Some("main.cxx".to_string());
        r.matched_source_code = Some("code".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```cpp\n"));
    }

    #[test]
    fn test_lang_hpp() {
        let mut r = make_empty_response();
        r.file_path = Some("main.hpp".to_string());
        r.matched_source_code = Some("code".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```cpp\n"));
    }

    #[test]
    fn test_lang_php() {
        let mut r = make_empty_response();
        r.file_path = Some("index.php".to_string());
        r.matched_source_code = Some("<?php echo 1;".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```php\n"));
    }

    #[test]
    fn test_lang_bash() {
        let mut r = make_empty_response();
        r.file_path = Some("run.sh".to_string());
        r.matched_source_code = Some("#!/bin/bash".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```bash\n"));
    }

    #[test]
    fn test_lang_bash_ext() {
        let mut r = make_empty_response();
        r.file_path = Some("run.bash".to_string());
        r.matched_source_code = Some("echo hi".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```bash\n"));
    }

    #[test]
    fn test_lang_hcl() {
        let mut r = make_empty_response();
        r.file_path = Some("main.tf".to_string());
        r.matched_source_code = Some("resource \"aws_s3_bucket\" {}".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```hcl\n"));
    }

    #[test]
    fn test_lang_yaml() {
        let mut r = make_empty_response();
        r.file_path = Some("config.yaml".to_string());
        r.matched_source_code = Some("key: value".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```yaml\n"));
    }

    #[test]
    fn test_lang_yml() {
        let mut r = make_empty_response();
        r.file_path = Some("config.yml".to_string());
        r.matched_source_code = Some("key: value".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```yaml\n"));
    }

    #[test]
    fn test_lang_json() {
        let mut r = make_empty_response();
        r.file_path = Some("data.json".to_string());
        r.matched_source_code = Some("{}".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```json\n"));
    }

    #[test]
    fn test_lang_unknown_uses_extension() {
        let mut r = make_empty_response();
        r.file_path = Some("script.lua".to_string());
        r.matched_source_code = Some("print('hi')".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```lua\n"));
    }

    #[test]
    fn test_lang_no_file_path_uses_text() {
        let mut r = make_empty_response();
        r.matched_source_code = Some("some code".to_string());
        let md = to_markdown(&r);
        assert!(md.contains("```text\n"));
    }

    #[test]
    fn test_matched_source_code_section_present() {
        let r = make_full_response();
        let md = to_markdown(&r);
        assert!(md.contains("## マッチしたソースコード"));
        assert!(md.contains("SELECT * FROM users"));
    }

    #[test]
    fn test_matched_source_code_section_absent_when_none() {
        let r = make_empty_response();
        let md = to_markdown(&r);
        assert!(!md.contains("## マッチしたソースコード"));
    }

    #[test]
    fn test_matched_source_code_section_absent_when_whitespace() {
        let mut r = make_empty_response();
        r.matched_source_code = Some("   \n  ".to_string());
        let md = to_markdown(&r);
        assert!(!md.contains("## マッチしたソースコード"));
    }

    #[test]
    fn test_analysis_section() {
        let r = make_full_response();
        let md = to_markdown(&r);
        assert!(md.contains("## 詳細解析"));
        assert!(md.contains("detailed analysis"));
    }

    #[test]
    fn test_poc_section_present() {
        let r = make_full_response();
        let md = to_markdown(&r);
        assert!(md.contains("## PoC"));
        assert!(md.contains("curl http://evil"));
    }

    #[test]
    fn test_poc_section_absent_when_empty() {
        let r = make_empty_response();
        let md = to_markdown(&r);
        assert!(!md.contains("## PoC"));
    }

    #[test]
    fn test_scratchpad_section_present() {
        let r = make_full_response();
        let md = to_markdown(&r);
        assert!(md.contains("## 解析ノート"));
        assert!(md.contains("thinking notes"));
    }

    #[test]
    fn test_scratchpad_section_absent_when_empty() {
        let r = make_empty_response();
        let md = to_markdown(&r);
        assert!(!md.contains("## 解析ノート"));
    }
}
